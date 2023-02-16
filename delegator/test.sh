set -x

cargo contract upload --manifest-path=accumulator/Cargo.toml --suri //Alice
cargo contract upload --manifest-path=adder/Cargo.toml --suri //Alice
cargo contract upload --manifest-path=subber/Cargo.toml --suri //Alice

ACCUMULATOR_HASH=$(cat target/ink/accumulator/*.contract | jq -r .source.hash)
ADDER_HASH=$(cat target/ink/adder/*.contract | jq -r .source.hash)
SUBBER_HASH=$(cat target/ink/subber/*.contract | jq -r .source.hash)

cargo contract instantiate --manifest-path=Cargo.toml --suri //Alice --args 0 0 $ACCUMULATOR_HASH $ADDER_HASH $SUBBER_HASH

