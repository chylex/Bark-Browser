#!/bin/bash
set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")"
cd ..

mkdir -p out
cargo build --release
cd target/release

VERSION=$(./bark --version)
ARCH=$(uname -m)

if [[ "$ARCH" == "x86_64" ]]; then
  ARCH="x64"
elif [[ "$ARCH" == "aarch64" ]]; then
  ARCH="arm64"
fi

tar czf "../../out/bark-${VERSION}-linux-${ARCH}.tar.gz" --owner=0 --group=0 -- bark
