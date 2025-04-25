# Copyright © Aptos Foundation
# SPDX-License-Identifier: Apache-2.0
# This script installs the necessary dependencies to build in Aptos.

# Set strict error handling
$ErrorActionPreference = 'Stop'
Set-PSDebug -Trace 1

# Check Windows version and type
$osVersion = [System.Environment]::OSVersion.Version
$isServer = (Get-WmiObject -Class Win32_OperatingSystem).ProductType -eq 3

# Define minimum versions
$minClientVersion = [Version]"10.0.17763" # Windows 10 1809
$minServerVersion = [Version]"10.0.17763" # Windows Server 2019

# Check version compatibility
if ($isServer) {
    if ($osVersion -lt $minServerVersion) {
        Write-Error "This script requires Windows Server 2019 or later. Current version: $($osVersion.ToString())"
        exit 1
    }
    Write-Host "Windows Server $($osVersion.ToString()) detected."
} else {
    if ($osVersion -lt $minClientVersion) {
        Write-Error "This script requires Windows 10 version 1809 (October 2018 Update) or later. Current version: $($osVersion.ToString())"
        exit 1
    }
    Write-Host "Windows $($osVersion.ToString()) detected."
}

# Check if winget is available
try {
    $wingetVersion = winget --version
    Write-Host "Winget version $wingetVersion is installed."
}
catch {
    Write-Host "Winget is not installed. Attempting to install..."
    
    # Check if App Installer is installed
    $appInstaller = Get-AppxPackage -Name Microsoft.DesktopAppInstaller
    if (-not $appInstaller) {
        Write-Host "Installing App Installer..."
        try {
            # Download and install App Installer
            $url = "https://aka.ms/getwinget"
            $output = "$env:TEMP\Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle"
            Invoke-WebRequest -Uri $url -OutFile $output
            Add-AppxPackage -Path $output
            Remove-Item $output
        }
        catch {
            Write-Error "Failed to install App Installer. Please install it manually from: https://aka.ms/getwinget"
            exit 1
        }
    }
    
    # Verify winget installation
    try {
        $wingetVersion = winget --version
        Write-Host "Winget version $wingetVersion successfully installed."
    }
    catch {
        Write-Error "Winget installation failed. Please install it manually from the Microsoft Store."
        exit 1
    }
}

# Change to the root directory of the project
try
{
    Set-Location (Split-Path -Parent $MyInvocation.MyCommand.Path) | Out-Null
    Set-Location '..' -ErrorAction Stop
}
catch
{
    Write-Error "Failed to change to project root directory: $_"
    exit 1
}

# Define required packages
$requiredPackages = @(
    "Microsoft.UI.Xaml.2.8",
    "Microsoft.VisualStudio.2022.BuildTools",
    "ShiningLight.OpenSSL.Dev",
    "LLVM.LLVM",
    "Rustlang.Rustup",
    "Git.Git"
)

# Function to check if a package is installed
function Test-PackageInstalled
{
    param (
        [string]$PackageId
    )
    $result = winget list --id $PackageId --accept-source-agreements
    return $result -match $PackageId
}

# Install each package
foreach ($package in $requiredPackages)
{
    try
    {
        if (Test-PackageInstalled -PackageId $package)
        {
            Write-Host "$package is already installed."
        }
        else
        {
            Write-Host "Installing $package..."
            winget install $package --accept-package-agreements --accept-source-agreements
        }
    }
    catch
    {
        Write-Error "Error with $package : $_"
        exit 1
    }
}

# VCLibs for C++, this one is special and has to be run separate
try
{
    Write-Host "Installing VCLibs"
    winget install "Microsoft.VCLibs.Desktop.14" --force --accept-package-agreements --accept-source-agreements --override "--add Microsoft.VisualStudio.Workload.VCTools --includeRecommended --quiet --wait --norestart"
}
catch
{
    Write-Error "Error with $package : $_"
    exit 1
}

Write-Host "All dependencies installed successfully!"
Write-Host "Please restart your terminal to use the tools"
