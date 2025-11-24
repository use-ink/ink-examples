# ERC-721 Non-Fungible Token (ink! v6)

This project implements the ERC-721 standard using the ink! v6 framework, compiled for the PolkaVM RISC-V architecture.

While the previous ERC-20 example dealt with "Fungible" tokens (where every token is identical in value), this contract implements "Non-Fungible" tokens (NFTs). In this standard, every token is unique and identified by a specific ID. This is commonly used for digital collectibles, real estate, or unique gaming items.

## Project Structure

- **Cargo.toml**: The configuration file managing dependencies. It specifically targets `ink` version `6.0.0-beta` and the `parity-scale-codec` for data encoding.
- **lib.rs**: The source code containing the contract logic.

## Prerequisites

To build and run this contract, your environment must support the ink! v6 toolchain.

1. Add the Rust source code (required for compiling the standard library for RISC-V):
   ```bash
   rustup component add rust-src
   ```

2. Install the specific Beta version of the contract compiler:
   ```bash
   cargo install --force --locked --version 6.0.0-beta cargo-contract
   ```

## Building the Contract

To compile the source code:

```bash
cargo contract build
```

Upon success, the artifacts will be generated in `target/ink/`:
- **erc721.contract**: The deployment bundle.
- **erc721.polkavm**: The raw RISC-V machine code.
- **erc721.json**: The ABI metadata.

## Architectural Overview

### 1. The Concept of Uniqueness
In an ERC-20 contract, we only tracked how *much* money someone had (`Balance`). In ERC-721, we must track *which* specific items someone owns (`TokenId`).

We define `TokenId` as a `u32` (a simple number). Token #1 is distinct from Token #2, and they cannot be merged.

### 2. Storage Layout
The contract uses several `Mapping` structures to maintain the state of the system:

- **token_owner**: `Mapping<TokenId, H160>`
  This is the source of truth. It maps a specific Token ID to the address of its owner. If an ID does not exist in this map, the token has not been minted (or was burned).

- **owned_tokens_count**: `Mapping<H160, u32>`
  Tracks the total number of tokens owned by an account. This is useful for external applications to know how many items a user has without counting them one by one.

- **token_approvals**: `Mapping<TokenId, H160>`
  Stores specific permissions. If Alice wants to sell Token #1 on a marketplace, she "approves" the marketplace contract only for Token #1.

- **operator_approvals**: `Mapping<(Owner, Operator), bool>`
  Stores broad permissions. If Alice wants a game to manage her entire inventory, she sets the game contract as an "operator". This allows the operator to transfer *any* token owned by Alice.

### 3. Events
The contract emits events to ensure off-chain indexers can track the history of every asset:
- **Transfer**: Emitted when ownership changes. This includes Minting (from `None` to User) and Burning (from User to `None`).
- **Approval**: Emitted when a single token ID is approved for a spender.
- **ApprovalForAll**: Emitted when an operator is authorized or revoked for an owner.

### 4. Error Handling
The contract defines a custom `Error` enum to handle logic failures gracefully:
- **TokenExists**: Returned when trying to mint an ID that has already been created.
- **TokenNotFound**: Returned when trying to transfer or query an ID that does not exist.
- **NotOwner**: Returned when a user tries to transfer a token they do not own.
- **NotApproved**: Returned when a spender tries to move a token they have not been authorized to use.

## Functional Logic

### Minting
The `mint(id)` function creates a new token.
1. It checks if the `id` already exists.
2. If not, it assigns the `id` to the caller in `token_owner`.
3. It increments the caller's `owned_tokens_count`.
4. It emits a `Transfer` event.

### Transferring
The `transfer(to, id)` and `transfer_from(from, to, id)` functions move assets.
1. They verify the token exists.
2. They verify the caller has permission (is the Owner, is the Approved Spender, or is an Operator).
3. They clear any specific approvals for that token (approvals do not carry over to the new owner).
4. They update the `token_owner` map and adjust the `owned_tokens_count` for both parties.

### The Approval System
This contract implements a two-tier approval system:
1. **Single Item Approval**: `approve(to, id)` authorizes a transfer for one specific token ID.
2. **Operator Approval**: `set_approval_for_all(operator, approved)` authorizes the operator to manage the entire portfolio of the caller. This is efficient for marketplaces or gaming logic where multiple transfers occur.

## Testing

The project includes unit tests to verify the core logic without deploying to a chain.

```bash
cargo test
```

These tests cover:
- **Minting**: Ensuring tokens are assigned to the creator.
- **Transferring**: Ensuring ownership updates correctly and balances change.
- **Security**: Verifying that unauthorized users cannot transfer tokens they do not own.
