#!/bin/bash
set -euo pipefail

# General
sudo apt-get install -y         \
     apt-transport-https        \
     build-essential            \
     ca-certificates            \
     gdb                        \
     gnupg                      \
     software-properties-common \
     wget

# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
rustup component add rust-src

# CMake
wget -O - https://apt.kitware.com/keys/kitware-archive-latest.asc 2>/dev/null | sudo apt-key add -
sudo apt-add-repository 'deb https://apt.kitware.com/ubuntu/ bionic main'
sudo apt-get update
sudo apt-get install -y cmake
