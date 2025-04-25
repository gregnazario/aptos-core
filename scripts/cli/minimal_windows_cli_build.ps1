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

# Function to ensure App Installer service is running
function Start-AppInstallerService {
    $service = Get-Service -Name "AppInstaller" -ErrorAction SilentlyContinue
    if ($service -and $service.Status -ne "Running") {
        Write-Host "Starting App Installer service..."
        Start-Service -Name "AppInstaller"
        Start-Sleep -Seconds 2
    }
}

# Check if winget is available
try {
    $wingetVersion = winget --version
    Write-Host "Winget version $wingetVersion is installed."
}
catch {
    Write-Host "Winget is not installed. Installing automatically..."
    
    # Ensure App Installer service is running
    Start-AppInstallerService
    
    # Check if App Installer is installed
    $appInstaller = Get-AppxPackage -Name Microsoft.DesktopAppInstaller
    if (-not $appInstaller) {
        Write-Host "Installing App Installer..."
        try {
            # Download and install App Installer with silent parameters
            $url = "https://aka.ms/getwinget"
            $output = "$env:TEMP\Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle"
            
            # Download with retry logic
            $retryCount = 3
            $retryDelay = 5
            for ($i = 1; $i -le $retryCount; $i++) {
                try {
                    Invoke-WebRequest -Uri $url -OutFile $output -UseBasicParsing
                    break
                }
                catch {
                    if ($i -eq $retryCount) { throw }
                    Write-Host "Download attempt $i failed. Retrying in $retryDelay seconds..."
                    Start-Sleep -Seconds $retryDelay
                }
            }
            
            # Install with silent parameters
            Add-AppxPackage -Path $output -ErrorAction Stop
            Remove-Item $output -Force
            
            # Wait for installation to complete
            Start-Sleep -Seconds 5
            
            # Refresh environment variables
            $env:Path = [System.Environment]::GetEnvironmentVariable("Path", "Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path", "User")
        }
        catch {
            Write-Error "Failed to install App Installer automatically. Error: $_"
            exit 1
        }
    }
    
    # Verify winget installation with retry logic
    $retryCount = 3
    $retryDelay = 5
    for ($i = 1; $i -le $retryCount; $i++) {
        try {
            $wingetVersion = winget --version
            Write-Host "Winget version $wingetVersion successfully installed."
            break
        }
        catch {
            if ($i -eq $retryCount) {
                Write-Error "Winget installation failed after $retryCount attempts. Please check your system requirements."
                exit 1
            }
            Write-Host "Winget verification attempt $i failed. Retrying in $retryDelay seconds..."
            Start-Sleep -Seconds $retryDelay
        }
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
    winget install "Microsoft.VCLibs.Desktop.14" --accept-package-agreements --accept-source-agreements --override "--add Microsoft.VisualStudio.Workload.VCTools --includeRecommended --quiet --wait --norestart"
}
catch
{
    Write-Error "Error with $package : $_"
    exit 1
}

Write-Host "All dependencies installed successfully!"
Write-Host "Please restart your terminal to use the tools"
