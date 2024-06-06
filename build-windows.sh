#!/bin/bash
set -e

# Update toolchain
rustup target add x86_64-pc-windows-gnu

# Build the executable and move to working directory
cargo build --release --target x86_64-pc-windows-gnu
mv ./target/x86_64-pc-windows-gnu/release/$(basename $(pwd)).exe ./$(basename $(pwd)).exe

# Download rcedit binary for next step
if [ ! -f ./rcedit-x64.exe ]; then
	wget https://github.com/electron/rcedit/releases/download/v2.0.0/rcedit-x64.exe
fi

# Modify the executable to apply the icon
exec wine ./rcedit-x64.exe ./$(basename $(pwd)).exe --set-icon ./icon.ico
