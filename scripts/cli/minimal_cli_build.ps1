# Copyright © Aptos Foundation
# SPDX-License-Identifier: Apache-2.0

# Minimal setup script for building the Aptos CLI on Windows.
# Mirrors the philosophy of minimal_cli_build.sh: install only what's needed,
# skip what's already present, and be fast on CI.
#
# On GitHub Actions windows-2025 runners, Rust, MSVC, Git, CMake, and vcpkg
# are all pre-installed. The only thing that typically needs installing is
# OpenSSL via vcpkg.

$ErrorActionPreference = "Stop"

# ── 1. Rust ──────────────────────────────────────────────────────────────────
if (Get-Command cargo -ErrorAction SilentlyContinue) {
    Write-Host "cargo found, skipping Rust install"
} else {
    Write-Host "Installing Rust via rustup-init.exe"
    $rustupInit = "$env:TEMP\rustup-init.exe"
    [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
    Invoke-WebRequest -Uri "https://win.rustup.rs/x86_64" -OutFile $rustupInit
    & $rustupInit -y --default-toolchain stable
    # Add cargo to PATH for the rest of this session
    $env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"
    Write-Host "Rust installed: $(rustc --version)"
}

# ── 2. Git long paths ───────────────────────────────────────────────────────
# Windows has a 260-character path limit by default, which breaks large Rust
# monorepos. This is a no-op if already set.
Write-Host "Configuring git for long paths"
git config --global core.longpaths true

# ── 3. vcpkg ─────────────────────────────────────────────────────────────────
if ($env:VCPKG_INSTALLATION_ROOT -and (Test-Path $env:VCPKG_INSTALLATION_ROOT)) {
    # GitHub Actions runners set this variable
    $vcpkgRoot = $env:VCPKG_INSTALLATION_ROOT
    Write-Host "Using VCPKG_INSTALLATION_ROOT: $vcpkgRoot"
} elseif (Test-Path "C:\vcpkg") {
    $vcpkgRoot = "C:\vcpkg"
    Write-Host "Using existing vcpkg at $vcpkgRoot"
} else {
    Write-Host "Cloning and bootstrapping vcpkg"
    git clone https://github.com/microsoft/vcpkg.git C:\vcpkg
    & C:\vcpkg\bootstrap-vcpkg.bat -disableMetrics
    $vcpkgRoot = "C:\vcpkg"
}

# Export VCPKG_ROOT so openssl-sys (and other -sys crates) can find it
$env:VCPKG_ROOT = $vcpkgRoot
# Persist for later steps in GitHub Actions
if ($env:GITHUB_ENV) {
    "VCPKG_ROOT=$vcpkgRoot" | Out-File -FilePath $env:GITHUB_ENV -Append
}

# ── 4. OpenSSL via vcpkg ────────────────────────────────────────────────────
$vcpkgExe = Join-Path $vcpkgRoot "vcpkg.exe"

$opensslInstalled = & $vcpkgExe list openssl 2>&1
if ($opensslInstalled -match "openssl") {
    Write-Host "OpenSSL already installed in vcpkg, skipping"
} else {
    Write-Host "Installing OpenSSL via vcpkg"
    & $vcpkgExe install openssl:x64-windows-static-md --clean-after-build
    if ($LASTEXITCODE -ne 0) {
        Write-Error "Failed to install OpenSSL via vcpkg"
        exit 1
    }
}

# ── 5. MSVC build tools ─────────────────────────────────────────────────────
$vswhere = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
if (Test-Path $vswhere) {
    $vsPath = & $vswhere -latest -property installationPath
    Write-Host "MSVC build tools found: $vsPath"
} else {
    $msg = @"
MSVC build tools not found.
Please install Visual Studio Build Tools from:
  https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022
Select the 'Desktop development with C++' workload.
"@
    Write-Error $msg
    exit 1
}

Write-Host "Minimal CLI build environment ready"
