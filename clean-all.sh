#!/usr/bin/env bash

set -eu

cargo clean --manifest-path conditional-compilation/Cargo.toml
cargo clean --manifest-path contract-terminate/Cargo.toml
cargo clean --manifest-path contract-transfer/Cargo.toml
cargo clean --manifest-path custom-environment/Cargo.toml
cargo clean --manifest-path delegator/Cargo.toml
cargo clean --manifest-path delegator/adder/Cargo.toml
cargo clean --manifest-path delegator/subber/Cargo.toml
cargo clean --manifest-path delegator/accumulator/Cargo.toml
cargo clean --manifest-path delegator2/Cargo.toml
cargo clean --manifest-path delegator2/adder/Cargo.toml
cargo clean --manifest-path delegator2/subber/Cargo.toml
cargo clean --manifest-path delegator2/accumulator/Cargo.toml
cargo clean --manifest-path dns/Cargo.toml
cargo clean --manifest-path erc20/Cargo.toml
cargo clean --manifest-path erc721/Cargo.toml
cargo clean --manifest-path erc1155/Cargo.toml
cargo clean --manifest-path trait-flipper/Cargo.toml
cargo clean --manifest-path trait-erc20/Cargo.toml
cargo clean --manifest-path trait-incrementer/Cargo.toml
cargo clean --manifest-path payment-channel/Cargo.toml
cargo clean --manifest-path psp22-extension/Cargo.toml
cargo clean --manifest-path rand-extension/Cargo.toml
cargo clean --manifest-path flipper/Cargo.toml
cargo clean --manifest-path incrementer/Cargo.toml
cargo clean --manifest-path multisig/Cargo.toml
cargo clean --manifest-path upgradeable-contracts/forward-calls/Cargo.toml
cargo clean --manifest-path upgradeable-contracts/set-code-hash/Cargo.toml
cargo clean --manifest-path upgradeable-contracts/set-code-hash/updated-incrementer/Cargo.toml
