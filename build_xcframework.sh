#!/bin/bash
set -e

# Configuration
LIB_NAME="ios_security_guard"
HEADER_NAME="IOSSecurityGuard.h"

# Ensure cargo is installed
if ! command -v cargo &> /dev/null; then
    echo "Error: cargo is not installed."
    exit 1
fi

# Ensure cbindgen is installed
if ! command -v cbindgen &> /dev/null; then
    echo "Installing cbindgen..."
    cargo install cbindgen
fi

echo "Adding iOS targets..."
rustup target add aarch64-apple-ios
rustup target add aarch64-apple-ios-sim
# rustup target add x86_64-apple-ios # For older Intel simulators if needed

echo "Building for iOS Device (aarch64)..."
cargo build --release --target aarch64-apple-ios

echo "Building for iOS Simulator (aarch64)..."
cargo build --release --target aarch64-apple-ios-sim

echo "Generating C Header..."
cbindgen --config cbindgen.toml --crate $LIB_NAME --output $HEADER_NAME

echo "Creating XCFramework..."
rm -rf $LIB_NAME.xcframework
xcodebuild -create-xcframework \
    -library target/aarch64-apple-ios/release/lib$LIB_NAME.a \
    -headers $HEADER_NAME \
    -library target/aarch64-apple-ios-sim/release/lib$LIB_NAME.a \
    -headers $HEADER_NAME \
    -output $LIB_NAME.xcframework

echo "Done! $LIB_NAME.xcframework created."
