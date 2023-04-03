#!/usr/bin/env bash

set -eu

cd accumulator
cargo clean
cd ..
cd adder
cargo clean
cd ..
cd subber
cargo clean
cd ..
cargo clean
