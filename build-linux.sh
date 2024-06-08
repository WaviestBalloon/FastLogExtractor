#!/bin/bash
set -e

# Lowercase the directory name as Rust prefers snake or kebab case
basename=$(basename $(pwd))
lowercasebasename=$(echo $basename | tr '[:upper:]' '[:lower:]')

# Build the executable and move to working directory
# this is going to screw things up
cargo build --release
mv ./target/release/$lowercasebasename ./$basename
