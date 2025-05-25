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
$Env:VCPKG_ROOT = 'C:\vcpkg\'

# Detect architecture
$result = Get-WmiObject -Class Win32_Processor | Select-Object AddressWidth, Architecture | ConvertTo-Json -Compress
if ($result.Contains("64")) {
    if ($result.Contains("12")) {  # ARM64 architecture
        $ARCH="arm64"
    } else {
        $ARCH="x86_64"
    }
} else {
    $ARCH="x86"
}

# Get the version of the CLI from its Cargo.toml.
$VERSION = Get-Content $CARGO_PATH | Select-String -Pattern '^\w*version = "(\d*\.\d*.\d*)"' | % {"$($_.matches.groups[1])"}

# Install the developer tools
echo "Installing developer tools"
PowerShell -ExecutionPolicy Bypass -File scripts/windows_dev_setup.ps1

# Note: This is required to bypass openssl isssue on Windows.
echo "Installing OpenSSL"
if ($ARCH -eq "arm64") {
    vcpkg install openssl:arm64-windows-static-md --clean-after-build
} else {
    vcpkg install openssl:x64-windows-static-md --clean-after-build
}

# Build the CLI.
echo "Building release $VERSION of $NAME for Windows ($ARCH)"
cargo build -p $CRATE_NAME --profile cli

# Compress the CLI.
$ZIP_NAME="$NAME-$VERSION-Windows-$ARCH.zip"
echo "Compressing CLI to $ZIP_NAME"
Compress-Archive -Path target\cli\$CRATE_NAME.exe -DestinationPath $ZIP_NAME

