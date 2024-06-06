#!/bin/bash
set -e

# Update toolchain
rustup target add x86_64-pc-windows-gnu

# Build the executable and move to working directory
# this is going to screw things up
cargo build --release
mv ./target/release/release/$(basename $(pwd)).exe ./$(basename $(pwd)).exe
