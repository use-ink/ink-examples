#!/bin/bash

set -x

cargo contract build
cargo contract build --manifest-path=../../flipper/Cargo.toml
FLIPPER=$(cargo contract instantiate --manifest-path=../../flipper/Cargo.toml --suri //Alice --skip-confirm --args true --output-json --salt 14 | jq -r .contract)
PROXY=$(cargo contract instantiate --manifest-path=./Cargo.toml --suri //Alice --skip-confirm --args $FLIPPER --output-json | jq -r .contract)
cargo contract call --manifest-path=../../flipper/Cargo.toml --suri //Alice --skip-confirm --message get --contract $PROXY --dry-run
