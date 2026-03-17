#!/bin/sh
# Copyright © Aptos Foundation
# Parts of the project are originally copyright © Meta Platforms, Inc.
# SPDX-License-Identifier: Apache-2.0

has_command() {
  command -v "$1" > /dev/null 2>&1
}

# Check if a deb package is installed (for apt-based systems)
deb_installed() {
  dpkg -s "$1" > /dev/null 2>&1
}

# Filter a list of deb packages down to only the ones not yet installed.
# Usage: missing=$(filter_missing_debs pkg1 pkg2 pkg3)
filter_missing_debs() {
  _missing=""
  for _pkg in "$@"; do
    if ! deb_installed "$_pkg"; then
      _missing="$_missing $_pkg"
    fi
  done
  echo "$_missing"
}

# Run a command with sudo if not already root.
pkg_install() {
  if [ "$(id -u)" -eq 0 ]; then
    "$@"
  else
    sudo "$@"
  fi
}

# This script is used to set up a minimal environment for building the Aptos CLI and other tools.
# The `dev_setup.sh` script is way too complex, and too hard to figure out what is actually happening.  This script
# simplifies the process

# TODO: Do we need to add `ca-certificates`, `curl`, `unzip`, `wget`
# Install rustup
if ! has_command cargo; then
  if has_command curl; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  elif has_command wget; then
    wget -qO- https://sh.rustup.rs | sh -s -- -y
  else
    echo "Unable to install rustup, neither curl nor wget found. Abort"
    exit 1
  fi
fi

OS="$(uname)"

case "$OS" in
  Linux)
    if has_command apt-get; then
      # Ubuntu / Debian based APT-GET
      NEEDED=$(filter_missing_debs build-essential pkgconf libssl-dev git libudev-dev lld libdw-dev clang llvm cmake)
      if [ -n "$NEEDED" ]; then
        echo "Installing missing packages:$NEEDED"
        pkg_install apt-get update
        pkg_install apt-get install -y --no-install-recommends $NEEDED
      else
        echo "All apt packages already installed, skipping"
      fi
    elif has_command dnf; then
      # RHEL / CentOS based DNF
      # This depends on the OS!
      # Source the os-release file to parse it
      . /etc/os-release

      # Check if it's Rocky
      if [ "$ID" = "rocky" ]; then
        echo "Rocky Linux detected"
        pkg_install dnf install -y gcc gcc-c++ make pkgconf openssl-devel git systemd-devel lld elfutils-devel clang llvm cmake
      else
        pkg_install dnf install -y gcc gcc-c++ make pkgconf openssl-devel git rust-libudev-devel lld elfutils-devel clang llvm cmake
      fi
    elif has_command yum; then
      # RHEL / CentOS based YUM
      pkg_install yum install -y gcc gcc-c++ make pkgconf openssl-devel git rust-libudev-devel lld elfutils-devel clang llvm cmake
    elif has_command pacman; then
      # Arch based PACMAN
      pkg_install pacman -S --needed --noconfirm base-devel pkgconf openssl git lld clang llvm cmake
    elif has_command apk; then
      # Alpine based APK
      pkg_install apk add --no-cache alpine-sdk coreutils pkgconfig openssl-dev git lld elfutils-dev clang clang-dev llvm cmake libc-dev
    elif has_command zypper; then
      # OpenSUSE zypper
      pkg_install zypper install -y gcc gcc-c++ make pkg-config libopenssl-devel git libudev-devel lld libdw-devel clang llvm cmake
    elif has_command emerge; then
      # Gentoo Emerge
      pkg_install emerge --sync
      pkg_install emerge --ask=n sys-devel/gcc dev-libs/openssl dev-vcs/git dev-lang/rust llvm-core/clang
    elif has_command xbps-install; then
      # Void linux xbps
      pkg_install xbps-install -y gcc make pkg-config git lld elfutils-devel clang llvm cmake
    else
      echo "Unable to find supported Linux package manager (apt-get, dnf, yum, zypper, xbps or pacman). Abort"
      exit 1
    fi
  ;;
  Darwin)
    # macOS
    brew install pkgconfig openssl git llvm cmake
  ;;
  FreeBSD)
    # FreeBSD
    pkg_install pkg install -y gcc gmake binutils pkgconf git openssl cmake llvm hidapi
  ;;
  *)
    echo "Unknown OS. Abort."
    exit 1
  ;;
esac
