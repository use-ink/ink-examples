#!/usr/bin/env bash

set -eu

cargo contract build --manifest-path accumulator/Cargo.toml --release
cargo contract build --manifest-path adder/Cargo.toml --release
cargo contract build --manifest-path subber/Cargo.toml --release
cargo contract build --release
