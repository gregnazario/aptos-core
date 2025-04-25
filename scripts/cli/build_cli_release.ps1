# Copyright © Aptos Foundation
# SPDX-License-Identifier: Apache-2.0

###########################################
# Build and package a release for the CLI #
###########################################

# Note: This must be run from the root of the aptos-core repository.

# Set up basic variables.
$NAME="aptos-cli"
$CRATE_NAME="aptos"
$CARGO_PATH="crates\$CRATE_NAME\Cargo.toml"

# Get the version of the CLI from its Cargo.toml.
$VERSION = Get-Content $CARGO_PATH | Select-String -Pattern '^\w*version = "(\d*\.\d*.\d*)"' | % {"$($_.matches.groups[1])"}

# Install the build tools
echo "Installing build tools"
PowerShell -ExecutionPolicy Bypass -File scripts/cli/minimal_windows_cli_build.ps1

# Build the CLI.
echo "Building release $VERSION of $NAME for Windows"
~\.cargo\bin\cargo.exe build -p $CRATE_NAME --profile cli

# Compress the CLI.
$ZIP_NAME="$NAME-$VERSION-Windows-x86_64.zip"
echo "Compressing CLI to $ZIP_NAME"
Compress-Archive -Path target\cli\$CRATE_NAME.exe -DestinationPath $ZIP_NAME

