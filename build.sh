#!/bin/bash

# Build for all architectures
cargo build --release || exit 1
cross build --target x86_64-pc-windows-gnu --release || exit 1
cross build --target x86_64-apple-darwin --release || exit 1
cross build --target aarch64-apple-darwin --release || exit 1
