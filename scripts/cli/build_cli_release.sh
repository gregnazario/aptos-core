#!/bin/bash
# Copyright © Aptos Foundation
# SPDX-License-Identifier: Apache-2.0

###########################################
# Build and package a release for the CLI #
###########################################

# Note: This must be run from the root of the aptos-core repository

set -e

NUM_ARGS="$#"
NAME='aptos-cli'
CRATE_NAME='aptos'
CARGO_PATH="crates/$CRATE_NAME/Cargo.toml"
PLATFORM_NAME="$1"
EXPECTED_VERSION="$2"
SKIP_RELEASE_CHECK="$3"

# Grab system information
ARCH=$(uname -m)
OS=$(uname -s)
VERSION=$(sed -n '/^\w*version = /p' "$CARGO_PATH" | sed 's/^.*=[ ]*"//g' | sed 's/".*$//g')

echo "Building for Architecture: '$ARCH' OS: '$OS' Platform: '$PLATFORM_NAME' Version: '$VERSION'"

if [ "$NUM_ARGS" -eq 0 ]; then
  echo "No arguments provided. Please provide platform name then expected version.";
  exit 120;
elif [ "$NUM_ARGS" -eq 2 ]; then
  echo "Checking if CLI version was already released."
  # Do nothing
elif [ "$NUM_ARGS" -eq 3 ]; then
  # This is skipping release check
  echo "Skipping check if CLI version was already released."
else
  echo "Wrong number of arguments provided. Please provide platform name then expected version."
  exit 121
fi

# Check that the version is well-formed, note that it should already be correct, but this double checks it
if ! [[ "$EXPECTED_VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
  echo "Inputted version '$EXPECTED_VERSION' is malformed, must be of the form '^[0-9]+\.[0-9]+\.[0-9]+$'"
  exit 1
fi

# Check that the version matches the Cargo.toml
if [[ "$EXPECTED_VERSION" != "$VERSION" ]]; then
  echo "Wanted to release for '$EXPECTED_VERSION', but Cargo.toml says the version is '$VERSION'"
  exit 2
fi

# Check that the release doesn't already exist (we're using Linux from now on)
export CURL_RESP=`curl -s --stderr /dev/null --output /dev/null --head -f "https://github.com/aptos-labs/aptos-core/releases/download/aptos-cli-v$EXPECTED_VERSION/aptos-cli-$EXPECTED_VERSION--x86_64.zip"`

if [[ -z "$CURL_RESP" && -z "$SKIP_RELEASE_CHECK" ]]; then
  echo "Version '$EXPECTED_VERSION' already released"
  exit 3
fi

echo "Building release $VERSION of $NAME for $OS-$PLATFORM_NAME on $ARCH"
cargo build -p "$CRATE_NAME" --profile cli

cd target/cli/

# Compress the CLI
ZIP_NAME="$NAME-$VERSION-$PLATFORM_NAME-$ARCH.zip"

echo "Zipping release: $ZIP_NAME"
zip "$ZIP_NAME" "$CRATE_NAME"
mv "$ZIP_NAME" ../..
