#syntax=docker/dockerfile:1.4

FROM rust AS rust-base
WORKDIR /aptos

RUN rm -f /etc/apt/apt.conf.d/docker-clean; echo 'Binary::apt::APT::Keep-Downloaded-Packages "true";' > /etc/apt/apt.conf.d/keep-cache

# Workaround: LLVM apt repo uses SHA1 signatures, but Trixie disabled SHA1 by default
# See https://github.com/llvm/llvm-project/issues/179148
RUN <<EOF
mkdir -p /etc/crypto-policies/back-ends/
cat > /etc/crypto-policies/back-ends/sequoia.config << 'CONFIGEOF'
[hash_algorithms]
sha1 = "always"

[asymmetric_algorithms]
rsa1024 = "always"
CONFIGEOF
EOF

# NOTE: the version of LLVM installed here MUST match the version of LLVM rustc
# uses internally, so we may need to upgrade this when upgrading Rust versions.
ARG CLANG_VERSION=21
RUN --mount=type=cache,target=/var/cache/apt,sharing=locked \
    --mount=type=cache,target=/var/lib/apt,sharing=locked \
    apt-get update && apt-get --no-install-recommends install -y \
        binutils \
        cmake \
        curl \
        git \
        gnupg \
        libdw-dev \
        libpq-dev \
        libssl-dev \
        libudev-dev \
        lsb-release \
        pkg-config \
        wget \
    && bash -c "$(wget -O - https://apt.llvm.org/llvm.sh)" llvm.sh ${CLANG_VERSION} \
    && update-alternatives --install /usr/bin/clang clang /usr/bin/clang-${CLANG_VERSION} 100 \
    && update-alternatives --install /usr/bin/clang++ clang++ /usr/bin/clang++-${CLANG_VERSION} 100 \
    && update-alternatives --install /usr/bin/lld lld /usr/bin/lld-${CLANG_VERSION} 100

### cargo-chef base — shared by the planner and all builder stages ###
FROM rust-base AS cargo-chef-base
RUN cargo install cargo-chef --locked

### Planner: scan workspace Cargo manifests and emit a dependency recipe ###
# This stage only needs Cargo files (no compiled artifacts), so it stays small
# and is invalidated only when Cargo.toml / Cargo.lock files change.
FROM cargo-chef-base AS chef-planner
COPY --link . /aptos/
RUN cargo chef prepare --recipe-path recipe.json

### Shared build environment — tooling only, no source ###
FROM cargo-chef-base AS builder-base

# Confirm that this Dockerfile is being invoked from an appropriate builder.
# See https://github.com/aptos-labs/aptos-core/pull/2471
# See https://github.com/aptos-labs/aptos-core/pull/2472
ARG BUILT_VIA_BUILDKIT
ENV BUILT_VIA_BUILDKIT $BUILT_VIA_BUILDKIT
RUN test -n "$BUILT_VIA_BUILDKIT" || (printf "===\nREAD ME\n===\n\nYou likely just tried run a docker build using this Dockerfile using\nthe standard docker builder (e.g. docker build). The standard docker\nbuild command uses a builder that does not respect our .dockerignore\nfile, which will lead to a build failure. To build, you should instead\nrun a command like one of these:\n\ndocker/docker-bake-rust-all.sh\ndocker/docker-bake-rust-all.sh indexer\n\nIf you are 100 percent sure you know what you're doing, you can add this flag:\n--build-arg BUILT_VIA_BUILDKIT=true\n\nFor more information, see https://github.com/aptos-labs/aptos-core/pull/2472\n\nThanks!" && false)

# cargo profile and features
ARG PROFILE
ENV PROFILE ${PROFILE}
ARG FEATURES
ENV FEATURES ${FEATURES}
ARG CARGO_TARGET_DIR
ENV CARGO_TARGET_DIR ${CARGO_TARGET_DIR}

RUN ARCHITECTURE=$(uname -m | sed -e "s/arm64/arm_64/g" | sed -e "s/aarch64/aarch_64/g") \
    && curl -LOs "https://github.com/protocolbuffers/protobuf/releases/download/v21.5/protoc-21.5-linux-$ARCHITECTURE.zip" \
    && unzip -o "protoc-21.5-linux-$ARCHITECTURE.zip" -d /usr/local bin/protoc \
    && unzip -o "protoc-21.5-linux-$ARCHITECTURE.zip" -d /usr/local 'include/*' \
    && chmod +x "/usr/local/bin/protoc" \
    && rm "protoc-21.5-linux-$ARCHITECTURE.zip"

RUN --mount=type=secret,id=GIT_CREDENTIALS,target=/root/.git_credentials \
    git config --global credential.helper store

# Copy cargo configuration so the cook steps below use the correct linker
# (.cargo/config.toml sets clang + lld for x86_64-unknown-linux-gnu).
COPY --link .cargo/ /aptos/.cargo/
COPY --link rust-toolchain.toml /aptos/rust-toolchain.toml

### aptos-node builder ###
FROM builder-base AS aptos-node-builder

# Cook dependencies: this layer is cached until Cargo.toml / Cargo.lock change.
# The performance profile requires an extra --config flag for cross-language LTO.
COPY --from=chef-planner /aptos/recipe.json recipe.json
RUN --mount=type=secret,id=GIT_CREDENTIALS,target=/root/.git-credentials \
    --mount=type=cache,target=/usr/local/cargo/git,id=node-builder-cargo-git-cache \
    --mount=type=cache,target=/usr/local/cargo/registry,id=node-builder-cargo-registry-cache \
    if [ "${PROFILE:-release}" = "performance" ]; then \
      cargo chef cook --profile=performance --config .cargo/performance.toml \
          -p aptos-node -p aptos-debugger --recipe-path recipe.json; \
    else \
      cargo chef cook --locked --profile=${PROFILE:-release} \
          -p aptos-node -p aptos-debugger --recipe-path recipe.json; \
    fi

# Build application code. .dockerignore excludes target/, so pre-compiled deps
# from the cook step above are preserved when source is copied in.
COPY --link . /aptos/
RUN --mount=type=secret,id=GIT_CREDENTIALS,target=/root/.git-credentials \
    --mount=type=cache,target=/usr/local/cargo/git,id=node-builder-cargo-git-cache \
    --mount=type=cache,target=/usr/local/cargo/registry,id=node-builder-cargo-registry-cache \
    docker/builder/build-with-feature.sh

### forge builder ###
FROM builder-base AS forge-builder

COPY --from=chef-planner /aptos/recipe.json recipe.json
RUN --mount=type=secret,id=GIT_CREDENTIALS,target=/root/.git-credentials \
    --mount=type=cache,target=/usr/local/cargo/git,id=forge-builder-cargo-git-cache \
    --mount=type=cache,target=/usr/local/cargo/registry,id=forge-builder-cargo-registry-cache \
    if [ "${PROFILE:-release}" = "performance" ]; then \
      cargo chef cook --profile=performance --config .cargo/performance.toml \
          -p aptos-forge-cli --recipe-path recipe.json; \
    else \
      cargo chef cook --locked --profile=${PROFILE:-release} \
          -p aptos-forge-cli --recipe-path recipe.json; \
    fi

COPY --link . /aptos/
RUN --mount=type=secret,id=GIT_CREDENTIALS,target=/root/.git-credentials \
    --mount=type=cache,target=/usr/local/cargo/git,id=forge-builder-cargo-git-cache \
    --mount=type=cache,target=/usr/local/cargo/registry,id=forge-builder-cargo-registry-cache \
    docker/builder/build-forge-cli.sh

### tools builder ###
FROM builder-base AS tools-builder

COPY --from=chef-planner /aptos/recipe.json recipe.json
RUN --mount=type=secret,id=GIT_CREDENTIALS,target=/root/.git-credentials \
    --mount=type=cache,target=/usr/local/cargo/git,id=tools-builder-cargo-git-cache \
    --mount=type=cache,target=/usr/local/cargo/registry,id=tools-builder-cargo-registry-cache \
    cargo chef cook --locked --profile=cli \
        -p aptos \
        -p aptos-faucet-service \
        -p aptos-openapi-spec-generator \
        -p aptos-telemetry-service \
        -p aptos-keyless-pepper-service \
        -p aptos-transaction-emitter \
        -p aptos-release-builder \
        --recipe-path recipe.json

COPY --link . /aptos/
RUN --mount=type=secret,id=GIT_CREDENTIALS,target=/root/.git-credentials \
    --mount=type=cache,target=/usr/local/cargo/git,id=tools-builder-cargo-git-cache \
    --mount=type=cache,target=/usr/local/cargo/registry,id=tools-builder-cargo-registry-cache \
    docker/builder/build-tools-with-cli-profile.sh

### indexer builder ###
FROM builder-base AS indexer-builder

COPY --from=chef-planner /aptos/recipe.json recipe.json
RUN --mount=type=secret,id=GIT_CREDENTIALS,target=/root/.git-credentials \
    --mount=type=cache,target=/usr/local/cargo/git,id=indexer-builder-cargo-git-cache \
    --mount=type=cache,target=/usr/local/cargo/registry,id=indexer-builder-cargo-registry-cache \
    if [ "${PROFILE:-release}" = "performance" ]; then \
      cargo chef cook --profile=performance --config .cargo/performance.toml \
          -p aptos-indexer-grpc-cache-worker \
          -p aptos-indexer-grpc-file-store \
          -p aptos-indexer-grpc-data-service \
          -p aptos-nft-metadata-crawler \
          -p aptos-indexer-grpc-file-checker \
          -p aptos-indexer-grpc-data-service-v2 \
          -p aptos-indexer-grpc-manager \
          -p aptos-indexer-grpc-gateway \
          --recipe-path recipe.json; \
    else \
      cargo chef cook --locked --profile=${PROFILE:-release} \
          -p aptos-indexer-grpc-cache-worker \
          -p aptos-indexer-grpc-file-store \
          -p aptos-indexer-grpc-data-service \
          -p aptos-nft-metadata-crawler \
          -p aptos-indexer-grpc-file-checker \
          -p aptos-indexer-grpc-data-service-v2 \
          -p aptos-indexer-grpc-manager \
          -p aptos-indexer-grpc-gateway \
          --recipe-path recipe.json; \
    fi

COPY --link . /aptos/
RUN --mount=type=secret,id=GIT_CREDENTIALS,target=/root/.git-credentials \
    --mount=type=cache,target=/usr/local/cargo/git,id=indexer-builder-cargo-git-cache \
    --mount=type=cache,target=/usr/local/cargo/registry,id=indexer-builder-cargo-registry-cache \
    docker/builder/build-indexer.sh
