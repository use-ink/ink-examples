<div align="center">
   <img src="./.images/ink-logo-glow.svg" alt="ink!" height="136" />
</div>

This repository contains a set of example contracts for ink!.

Have a look at the different examples to better understand how to use ink! to build your own Substrate smart contracts.

### Can I add a new example here?

Please don't add them here, but create a Pull Request to the `integration-tests/public` folder in [the ink! repository](https://github.com/paritytech/ink) instead.
The content of that folder is synchronized with this repository on new releases.

## Preparation

For building the example smart contracts found in this folder you will need to have [`cargo-contract`](https://github.com/paritytech/cargo-contract) installed.

```sh
cargo install cargo-contract --force
```

We use the `--force` to update to the most recent `cargo-contract` version.

## Build example contract and generate the contracts metadata

To build a single example and generate the contracts Wasm file, navigate to the root of the smart contract and run the following command:

`cargo contract build`

You should now have an optimized `<contract-name>.wasm` file, a `metadata.json` file and a `<contract-name>.contract` file in the `target` folder of your contract.
The `.contract` file combines the Wasm and metadata into one file and can be used for instantiation.

## Running front end dApp examples

1. Install [nodejs](https://nodejs.org/en/) and then install [pnpm](https://pnpm.io/) `npm install -g pnpm`
2. Install dependencies `pnpm i`
3. Run each example with `pnpm <contract-example-name>`. e.g. `pnpm flipper`
4. Visit [http://localhost:5173](http://localhost:5173) in your browser.

### Commands

* `pnpm basic-contract-caller`
* `pnpm contract-terminate`
* `pnpm contract-transfer`
* `pnpm erc20`
* `pnpm erc721`
* `pnpm flipper`
* `pnpm incrementer`

All examples are built with [useink](https://use.ink/frontend/overview), a React hooks library built by the ink! team.

## License

The examples in this folder are released into the public domain.
We hope they help you build something great with ink!.

See the [LICENSE file](LICENSE) in this folder for more details.
