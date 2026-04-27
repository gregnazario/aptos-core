# This is a docker bake file in HCL syntax.
# It provides a high-level mechenanism to build multiple dockerfiles in one shot.
# Check https://crazymax.dev/docker-allhands2-buildx-bake and https://docs.docker.com/engine/reference/commandline/buildx_bake/#file-definition for an intro.

variable "CI" {
  # whether this build runs in aptos-labs' CI environment which makes certain assumptions about certain registries being available to push to cache layers.
  # for local builds we simply default to relying on dockers local caching.
  default = "false"
}
variable "TARGET_CACHE_ID" {}
variable "BUILD_DATE" {}
// this is the full GIT_SHA - let's use that as primary identifier going forward
variable "GIT_SHA" {}

variable "GIT_BRANCH" {}

variable "GIT_TAG" {}

variable "GIT_CREDENTIALS" {}

variable "BUILT_VIA_BUILDKIT" {}

variable "GCP_DOCKER_ARTIFACT_REPO" {}

// NOTE: AWS ECR publishing has been disabled. Variables kept for reference.
variable "AWS_ECR_ACCOUNT_NUM" {
  default = ""
}

variable "TARGET_REGISTRY" {
  // must be "gcp" | "local" | "remote-all" | "remote" (deprecated, but kept for backwards compatibility. Same as "gcp"), informs which docker tags are being generated
  // NOTE: "remote-all" no longer publishes to ECR (now behaves same as "gcp"/"remote")
  default = CI == "true" ? "remote" : "local"
}

// ECR base URL kept for reference (no longer used)
variable "ecr_base" {
  default = "${AWS_ECR_ACCOUNT_NUM}.dkr.ecr.us-west-2.amazonaws.com/aptos"
}

variable "NORMALIZED_GIT_BRANCH_OR_PR" {}
variable "IMAGE_TAG_PREFIX" {}
variable "PROFILE" {
  // Cargo compilation profile
  default = "release"
}
variable "FEATURES" {
  // Cargo features to enable, as a comma separated string
}
variable "CARGO_TARGET_DIR" {
  // Cargo target directory
}

group "all" {
  targets = flatten([
    "validator",
    "tools",
    "faucet",
    "forge",
    "telemetry-service",
    "keyless-pepper-service",
    "indexer-grpc",
    "validator-testing",
    "nft-metadata-crawler",
  ])
}

group "forge-images" {
  targets = ["validator-testing", "tools", "forge"]
}

target "debian-base" {
  dockerfile = "docker/builder/debian-base.Dockerfile"
  contexts = {
    # Run `docker buildx imagetools inspect debian:trixie` to find the latest multi-platform hash
    debian = "docker-image://debian:trixie@sha256:3352c2e13876c8a5c5873ef20870e1939e73cb9a3c1aeba5e3e72172a85ce9ed"
  }
}

// Pinned rust image digest shared between builder-base and chef-planner.
locals {
  rust_image = "docker-image://rust:1.93.1-trixie@sha256:ecbe59a8408895edd02d9ef422504b8501dd9fa1526de27a45b73406d734d659"
}

target "builder-base" {
  dockerfile = "docker/builder/builder.Dockerfile"
  target     = "builder-base"
  context    = "."
  contexts = {
    # Run `docker buildx imagetools inspect rust:1.93.1-trixie` to find the latest multi-platform hash
    rust = local.rust_image
  }
  args = {
    PROFILE            = "${PROFILE}"
    FEATURES           = "${FEATURES}"
    CARGO_TARGET_DIR   = "${CARGO_TARGET_DIR}"
    BUILT_VIA_BUILDKIT = "true"
  }
  secret = [
    "id=GIT_CREDENTIALS"
  ]
}

// Planner stage: scans workspace Cargo manifests and emits recipe.json.
// Invalidated only when Cargo.toml / Cargo.lock files change.
target "chef-planner" {
  dockerfile = "docker/builder/builder.Dockerfile"
  target     = "chef-planner"
  context    = "."
  contexts = {
    rust = local.rust_image
  }
  cache-from = CI == "true" ? [
    "type=registry,ref=${GCP_DOCKER_ARTIFACT_REPO}/builder-cache:planner-${NORMALIZED_GIT_BRANCH_OR_PR}",
    "type=registry,ref=${GCP_DOCKER_ARTIFACT_REPO}/builder-cache:planner-main",
  ] : []
  cache-to = CI == "true" ? [
    "type=registry,ref=${GCP_DOCKER_ARTIFACT_REPO}/builder-cache:planner-${NORMALIZED_GIT_BRANCH_OR_PR},mode=max",
  ] : []
}

// Each builder target below follows the same cargo-chef pattern:
//   1. cook step   — compiles only deps (Docker layer, cached until Cargo.lock changes)
//   2. source copy — brings in real code without touching target/
//   3. build step  — recompiles only the crates that changed
//
// cache-from tries the branch/PR cache first, then falls back to main so PRs
// get warm deps without requiring a prior successful build on the same branch.
// cache-to writes with mode=max so intermediate cook layers are also cached.

target "aptos-node-builder" {
  dockerfile = "docker/builder/builder.Dockerfile"
  target     = "aptos-node-builder"
  context    = "."
  contexts = {
    builder-base = "target:builder-base"
    chef-planner = "target:chef-planner"
  }
  cache-from = CI == "true" ? [
    "type=registry,ref=${GCP_DOCKER_ARTIFACT_REPO}/builder-cache:node-${NORMALIZED_GIT_BRANCH_OR_PR}",
    "type=registry,ref=${GCP_DOCKER_ARTIFACT_REPO}/builder-cache:node-main",
  ] : []
  cache-to = CI == "true" ? [
    "type=registry,ref=${GCP_DOCKER_ARTIFACT_REPO}/builder-cache:node-${NORMALIZED_GIT_BRANCH_OR_PR},mode=max",
  ] : []
  secret = [
    "id=GIT_CREDENTIALS"
  ]
}

target "tools-builder" {
  dockerfile = "docker/builder/builder.Dockerfile"
  target     = "tools-builder"
  context    = "."
  contexts = {
    builder-base = "target:builder-base"
    chef-planner = "target:chef-planner"
  }
  cache-from = CI == "true" ? [
    "type=registry,ref=${GCP_DOCKER_ARTIFACT_REPO}/builder-cache:tools-${NORMALIZED_GIT_BRANCH_OR_PR}",
    "type=registry,ref=${GCP_DOCKER_ARTIFACT_REPO}/builder-cache:tools-main",
  ] : []
  cache-to = CI == "true" ? [
    "type=registry,ref=${GCP_DOCKER_ARTIFACT_REPO}/builder-cache:tools-${NORMALIZED_GIT_BRANCH_OR_PR},mode=max",
  ] : []
  secret = [
    "id=GIT_CREDENTIALS"
  ]
}

target "forge-builder" {
  dockerfile = "docker/builder/builder.Dockerfile"
  target     = "forge-builder"
  context    = "."
  contexts = {
    builder-base = "target:builder-base"
    chef-planner = "target:chef-planner"
  }
  cache-from = CI == "true" ? [
    "type=registry,ref=${GCP_DOCKER_ARTIFACT_REPO}/builder-cache:forge-${NORMALIZED_GIT_BRANCH_OR_PR}",
    "type=registry,ref=${GCP_DOCKER_ARTIFACT_REPO}/builder-cache:forge-main",
  ] : []
  cache-to = CI == "true" ? [
    "type=registry,ref=${GCP_DOCKER_ARTIFACT_REPO}/builder-cache:forge-${NORMALIZED_GIT_BRANCH_OR_PR},mode=max",
  ] : []
  secret = [
    "id=GIT_CREDENTIALS"
  ]
}

target "indexer-builder" {
  dockerfile = "docker/builder/builder.Dockerfile"
  target     = "indexer-builder"
  context    = "."
  contexts = {
    builder-base = "target:builder-base"
    chef-planner = "target:chef-planner"
  }
  cache-from = CI == "true" ? [
    "type=registry,ref=${GCP_DOCKER_ARTIFACT_REPO}/builder-cache:indexer-${NORMALIZED_GIT_BRANCH_OR_PR}",
    "type=registry,ref=${GCP_DOCKER_ARTIFACT_REPO}/builder-cache:indexer-main",
  ] : []
  cache-to = CI == "true" ? [
    "type=registry,ref=${GCP_DOCKER_ARTIFACT_REPO}/builder-cache:indexer-${NORMALIZED_GIT_BRANCH_OR_PR},mode=max",
  ] : []
  secret = [
    "id=GIT_CREDENTIALS"
  ]
}

target "_common" {
  contexts = {
    debian-base     = "target:debian-base"
    node-builder    = "target:aptos-node-builder"
    forge-builder   = "target:forge-builder"
    tools-builder   = "target:tools-builder"
    indexer-builder = "target:indexer-builder"
  }
  labels = {
    "org.label-schema.schema-version" = "1.0",
    "org.label-schema.build-date"     = "${BUILD_DATE}"
    "org.label-schema.git-sha"        = "${GIT_SHA}"
  }
  args = {
    PROFILE    = "${PROFILE}"
    FEATURES   = "${FEATURES}"
    GIT_SHA    = "${GIT_SHA}"
    GIT_BRANCH = "${GIT_BRANCH}"
    GIT_TAG    = "${GIT_TAG}"
    BUILD_DATE = "${BUILD_DATE}"
  }
  output     = ["type=image,compression=zstd,force-compression=true"]
}

target "validator-testing" {
  inherits   = ["_common"]
  dockerfile = "docker/builder/validator-testing.Dockerfile"
  target     = "validator-testing"
  tags       = generate_tags("validator-testing")
}

target "tools" {
  inherits   = ["_common"]
  dockerfile = "docker/builder/tools.Dockerfile"
  target     = "tools"
  tags       = generate_tags("tools")
}

target "forge" {
  inherits   = ["_common"]
  dockerfile = "docker/builder/forge.Dockerfile"
  target     = "forge"
  tags       = generate_tags("forge")
}

target "validator" {
  inherits   = ["_common"]
  dockerfile = "docker/builder/validator.Dockerfile"
  target     = "validator"
  tags       = generate_tags("validator")
}

target "tools" {
  inherits   = ["_common"]
  dockerfile = "docker/builder/tools.Dockerfile"
  target     = "tools"
  tags       = generate_tags("tools")
}
target "faucet" {
  inherits   = ["_common"]
  dockerfile = "docker/builder/faucet.Dockerfile"
  target     = "faucet"
  tags       = generate_tags("faucet")
}

target "telemetry-service" {
  inherits   = ["_common"]
  dockerfile = "docker/builder/telemetry-service.Dockerfile"
  target     = "telemetry-service"
  tags       = generate_tags("telemetry-service")
}

target "keyless-pepper-service" {
  inherits   = ["_common"]
  dockerfile = "docker/builder/keyless-pepper-service.Dockerfile"
  target     = "keyless-pepper-service"
  tags       = generate_tags("keyless-pepper-service")
}

target "indexer-grpc" {
  inherits   = ["_common"]
  dockerfile = "docker/builder/indexer-grpc.Dockerfile"
  target     = "indexer-grpc"
  tags       = generate_tags("indexer-grpc")
}

target "nft-metadata-crawler" {
  inherits   = ["_common"]
  target     = "nft-metadata-crawler"
  dockerfile = "docker/builder/nft-metadata-crawler.Dockerfile"
  tags       = generate_tags("nft-metadata-crawler")
}

function "generate_tags" {
  params = [target]
  // NOTE: ECR publishing has been disabled. "remote-all" now behaves same as "gcp"/"remote".
  // ECR tags kept as reference (previously included when TARGET_REGISTRY == "remote-all"):
  //   "${ecr_base}/${target}:${IMAGE_TAG_PREFIX}${GIT_SHA}",
  //   "${ecr_base}/${target}:${IMAGE_TAG_PREFIX}${NORMALIZED_GIT_BRANCH_OR_PR}",
  result = TARGET_REGISTRY == "remote-all" || TARGET_REGISTRY == "gcp" || TARGET_REGISTRY == "remote" ? [
    "${GCP_DOCKER_ARTIFACT_REPO}/${target}:${IMAGE_TAG_PREFIX}${GIT_SHA}",
    "${GCP_DOCKER_ARTIFACT_REPO}/${target}:${IMAGE_TAG_PREFIX}${NORMALIZED_GIT_BRANCH_OR_PR}",
    ] : [ // "local" or any other value
    "aptos-core/${target}:${IMAGE_TAG_PREFIX}${GIT_SHA}-from-local",
    "aptos-core/${target}:${IMAGE_TAG_PREFIX}from-local",
  ]
}
