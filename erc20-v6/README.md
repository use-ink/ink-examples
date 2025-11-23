# ERC-20 Token Smart Contract (ink! v6)

This project implements a standard Fungible Token (ERC-20) using the ink! v6 framework. It is designed to run on the PolkaVM RISC-V architecture, replacing the older WebAssembly (Wasm) execution environment.

This contract implements the industry-standard logic found in major cryptocurrencies, allowing for minting, transferring, and delegated spending of tokens.

## Project Structure

- **Cargo.toml**: The configuration file. It specifies the dependencies, strictly targeting `ink` version `6.0.0-beta` and the `parity-scale-codec` for data encoding.
- **lib.rs**: The source code containing the contract logic, storage layout, error handling, and event definitions.

## Prerequisites

To build and run this contract, your environment must support the ink! v6 toolchain. If you have not configured this yet, execute the following commands in your terminal:

1. Add the Rust source code (required for compiling the standard library for RISC-V):
   ```bash
   rustup component add rust-src
   ```

2. Install the specific Beta version of the contract compiler:
   ```bash
   cargo install --force --locked --version 6.0.0-beta cargo-contract
   ```

## Building the Contract

To compile the source code into a deployable package:

```bash
cargo contract build
```

Upon success, the following artifacts will be generated in the `target/ink/` directory:
- **erc20.contract**: The bundle containing both the RISC-V bytecode and the metadata. This is the file you upload to the blockchain.
- **erc20.polkavm**: The raw compiled RISC-V machine code.
- **erc20.json**: The ABI (Application Binary Interface) describing the methods and types used in the contract.

## Testing

Before deploying, it is recommended to run the unit tests included in the library. These tests simulate blockchain interaction to verify logic.

```bash
cargo test
```

## Code Overview

### 1. Address Format (H160)
In ink! v6, the standard address format has shifted to `H160` (20 bytes), which aligns with the Ethereum standard. This ensures compatibility with the underlying PolkaVM architecture.

### 2. Storage Layout
The contract persists data using the `Mapping` type, which functions like a key-value database:
- **balances**: Maps an Address (`H160`) to a Balance (`u128`).
- **allowances**: Uses a composite key of `(Owner, Spender)` mapped to a Balance. This tracks how many tokens a specific third party is allowed to withdraw from an owner's account.

### 3. Events
Events are signals emitted by the smart contract that are indexed by the blockchain. External applications (dApps) listen to these events to update their user interfaces.
- **Transfer**: Emitted when tokens are moved, minted, or burned.
- **Approval**: Emitted when an owner authorizes a spender.

### 4. Error Handling
Instead of using `panic!`, which crashes the contract execution, this implementation returns a `Result` type. This allows the contract to fail gracefully with specific error codes:
- **InsufficientBalance**: returned when a user tries to send more than they have.
- **InsufficientAllowance**: returned when a spender tries to withdraw more than they were authorized to use.

### 5. The Allowance Mechanism
The ERC-20 standard includes a workflow for delegated spending. This is commonly used by Decentralized Exchanges (DEXs):
1. **Approve**: User A calls `approve(User B, 100)`. This gives User B permission to spend 100 tokens.
2. **Transfer From**: User B calls `transfer_from(User A, User C, 100)`. The contract checks the allowance, moves the funds from A to C, and reduces the allowance.

## UI Integration Guide

To interact with this contract using a graphical interface (such as the Contracts UI or Polkadot.js Apps), follow these steps:

### Phase 1: Uploading
1. Open your contract explorer or UI.
2. Select the option to "Upload New Contract".
3. Upload the `erc20.contract` file found in `target/ink/`.

### Phase 2: Instantiation
Once uploaded, you must create an instance of the contract on the chain.
1. The UI will prompt you for the Constructor inputs.
2. **total_supply**: Enter the initial number of tokens to mint (e.g., `1000000`).
3. Sign and submit the transaction.
4. The contract is now live, and your account holds the total supply.

### Phase 3: Interaction
You can now call the methods defined in the contract:

**To Check Balance:**
1. Select the `balance_of` message.
2. Paste your address (or the address of another account).
3. Click "Read" to see the result.

**To Transfer Tokens:**
1. Select the `transfer` message.
2. **to**: Enter the recipient's H160 address.
3. **value**: Enter the amount to send.
4. Sign and submit the transaction.

**To Test Allowances:**
1. Use Account A to call `approve` with Account B's address and a value of `500`.
2. Switch to Account B.
3. Call `transfer_from`.
   - **from**: Account A.
   - **to**: Account C (or B).
   - **value**: `500`.
4. The transaction should succeed, moving funds from A to the destination, initiated by B.
