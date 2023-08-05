#!/bin/bash
set -euo pipefail

# General
sudo apt-get install -y         \
     apt-transport-https        \
     build-essential            \
     ca-certificates            \
     cmake                      \
     gdb                        \
     gnupg                      \
     software-properties-common \
     wget

# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
rustup component add rust-src
