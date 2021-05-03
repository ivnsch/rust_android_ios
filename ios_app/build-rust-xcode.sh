#!/bin/bash

# Ensure that `cargo` is in PATH, using the default location.
. "$HOME/.cargo/env"

set -x

# Go to repo's root
cd "${SRCROOT}/../"

# Build binaries
cargo +ios-arm64-1.46.0 build --target aarch64-apple-ios --release --lib
cargo build --target=x86_64-apple-ios --release

# Create fat binary
libtool -static -o ./ios_app/core/libcore ./target/aarch64-apple-ios/release/libcore.a ./target/x86_64-apple-ios/release/libcore.a
