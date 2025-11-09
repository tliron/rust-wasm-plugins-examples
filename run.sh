#!/bin/bash
set -e

# Enable LTO and strip debuginfo to create smaller Wasm files
CARGO_PROFILE_RELEASE_LTO=true \
CARGO_PROFILE_RELEASE_STRIP=debuginfo \
cargo build --package=plugin --target=wasm32-wasip2 --release

cargo run --package=host
