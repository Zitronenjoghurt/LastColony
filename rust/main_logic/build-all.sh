#!/bin/bash
cargo build
cargo build --release
cargo build --target=x86_64-pc-windows-gnu --verbose
cargo build --target=x86_64-pc-windows-gnu --verbose --release