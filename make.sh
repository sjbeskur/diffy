#!/bin/bash

echo "Building for aarch64-unknown-linux-gnu" 
echo "------------------------------------------------------------------------------------------------"
export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=/usr/bin/aarch64-linux-gnu-gcc
RUSTFLAGS='-C target-feature=+crt-static' cargo build --target aarch64-unknown-linux-gnu --release

echo "Building for x86_64-unknown-linux-gnu" 
echo "------------------------------------------------------------------------------------------------"
RUSTFLAGS='-C target-feature=+crt-static' cargo build --target x86_64-unknown-linux-gnu --release
