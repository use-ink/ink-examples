# ERC-1155 Multi-Token Standard (ink! v6)

This project implements the ERC-1155 standard using the ink! v6 framework for the PolkaVM RISC-V architecture.

ERC-1155 is known as the "Multi-Token Standard." Unlike ERC-20 (which requires a new contract for every new currency) or ERC-721 (which tracks unique items individually), ERC-1155 can manage any number of fungible and non-fungible token types within a single smart contract. This makes it the industry standard for gaming and complex financial applications.

## Project Structure

- **Cargo.toml**: The configuration file. It targets `ink` version `6.0.0-beta` and includes the `parity-scale-codec` for data encoding.
- **lib.rs**: The source code containing the hybrid storage logic and batch transfer mechanisms.

## Prerequisites

To build this contract, you must have the ink! v6 toolchain installed:

1. Add the Rust source code:
   ```bash
   rustup component add rust-src
   ```

2. Install the contract compiler (Beta):
   ```bash
   cargo install --force --locked --version 6.0.0-beta cargo-contract
   ```

## Building the Contract

Compile the source code into a deployable package:

```bash
cargo contract build
```

Artifacts generated in `target/ink/`:
- **erc1155.contract**: The deployment bundle.
- **erc1155.polkavm**: The raw RISC-V bytecode.
- **erc1155.json**: The ABI metadata.

## Architectural Concepts

### 1. Hybrid Storage
The core difference between this and previous standards is the Storage Key.
- ERC-20 Maps: `User Address -> Balance`
- ERC-721 Maps: `Token ID -> Owner Address`
- **ERC-1155 Maps: `(User Address, Token ID) -> Balance`**

This allows a single user to hold a balance of "100" for Token ID #1 (e.g., Gold Coins) and a balance of "1" for Token ID #2 (e.g., a Unique Sword) simultaneously in the same map.

### 2. Batch Operations
In older standards, if you wanted to send a sword, a shield, and some gold to a friend, you had to make 3 separate transactions. In ERC-1155, you can send all of them in a single transaction using "Batch Transfer." This significantly reduces gas costs and blockchain congestion.

## Function Reference

### Read-Only Functions

#### `balance_of(owner: H160, id: TokenId) -> Balance`
Returns the amount of tokens of a specific `id` owned by the `owner`. 
- If the token is Fungible (like currency), this returns the wallet balance.
- If the token is an NFT, this returns `1` if they own it, or `0` if they don't.

#### `balance_of_batch(owners: Vec<H160>, ids: Vec<TokenId>) -> Result<Vec<Balance>>`
Allows querying multiple balances in a single call.
- **Input**: A list of owners and a list of IDs.
- **Output**: A list of balances corresponding to the inputs.
- **Constraint**: The length of `owners` and `ids` must match.

#### `is_approved_for_all(owner: H160, operator: H160) -> bool`
Checks if an `operator` (e.g., a Marketplace contract or a Game server) is authorized to manage *all* assets belonging to `owner`.

---

### State-Changing Functions (Write)

#### `mint(id: TokenId, amount: Balance)`
Creates new tokens.
- **id**: The identifier of the token type (e.g., `1` for Gold, `2` for Silver).
- **amount**: How many to create.
- **Effect**: Increases the balance of the caller and emits a `TransferSingle` event (from `None` to Caller).

#### `set_approval_for_all(operator: H160, approved: bool)`
Grants or revokes permission for a third-party `operator` to manage the caller's tokens.
- **True**: Grants permission. The operator can now transfer any of the caller's tokens.
- **False**: Revokes permission.
- **Note**: Unlike ERC-20, ERC-1155 typically does not support "approving" a specific amount of a specific token. It uses a "trust the operator with everything" model, which is more efficient for games.

#### `safe_transfer_from(from: H160, to: H160, id: TokenId, amount: Balance)`
Transfers a specific amount of a specific token type.
- **Checks**:
  1. Is the caller the `from` account OR an approved operator?
  2. Does the `from` account have enough balance?
- **Effect**: Deducts balance from sender, adds to receiver, and emits `TransferSingle`.

#### `safe_batch_transfer_from(from: H160, to: H160, ids: Vec<TokenId>, amounts: Vec<Balance>)`
The signature feature of ERC-1155. Transfers multiple token types at once.
- **ids**: A list of Token IDs to move (e.g., `[1, 2, 5]`).
- **amounts**: A list of amounts matching the IDs (e.g., `[100, 1, 50]`).
- **Constraint**: `ids.len()` must equal `amounts.len()`.
- **Effect**: Updates all balances atomically. If any part of the transfer fails (e.g., insufficient balance for the 3rd item), the *entire* transaction fails. Emits `TransferBatch`.

## Events

### `TransferSingle`
Emitted when a single token type is transferred.
- Contains: `operator`, `from`, `to`, `id`, `value`.

### `TransferBatch`
Emitted when a batch transfer occurs.
- Contains: `operator`, `from`, `to`, `ids` (list), `values` (list).
- This is highly efficient for indexers, as they only need to process one event for multiple asset movements.

### `ApprovalForAll`
Emitted when an operator is approved or revoked.

## Error Handling

The contract returns a `Result` type with the following custom errors:
- **NotOwner**: Caller tries to transfer tokens they don't control.
- **NotApproved**: Caller is not the owner and not an authorized operator.
- **InsufficientBalance**: Sender tries to send more than they have.
- **BatchSizeMismatch**: Passed arrays (IDs and Amounts) have different lengths.

## Testing

Unit tests are included to verify the logic.

```bash
cargo test
```

These tests confirm:
- **Minting**: Correctly updates balances.
- **Batch Transfers**: Successfully moves multiple assets and updates multiple storage entries in one go.
