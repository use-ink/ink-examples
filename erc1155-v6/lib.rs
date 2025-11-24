#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod erc1155 {
    use ink::storage::Mapping;
    use ink::primitives::H160;
    use ink::prelude::vec::Vec; // We need Vectors for batch operations

    pub type TokenId = u32;

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        NotOwner,
        NotApproved,
        InsufficientBalance,
        BatchSizeMismatch, // Arrays must have same length
    }

    pub type Result<T> = core::result::Result<T, Error>;

    // --- Events ---

    #[ink(event)]
    pub struct TransferSingle {
        #[ink(topic)]
        operator: H160,
        #[ink(topic)]
        from: Option<H160>,
        #[ink(topic)]
        to: Option<H160>,
        id: TokenId,
        value: Balance,
    }

    #[ink(event)]
    pub struct ApprovalForAll {
        #[ink(topic)]
        owner: H160,
        #[ink(topic)]
        operator: H160,
        approved: bool,
    }

    #[ink(event)]
    pub struct TransferBatch {
        #[ink(topic)]
        operator: H160,
        #[ink(topic)]
        from: Option<H160>,
        #[ink(topic)]
        to: Option<H160>,
        ids: Vec<TokenId>,
        values: Vec<Balance>,
    }

    // --- Storage ---

    #[ink(storage)]
    pub struct Erc1155 {
        /// Maps (Owner, TokenId) -> Balance
        balances: Mapping<(H160, TokenId), Balance>,
        /// Maps (Owner, Operator) -> Approved?
        operator_approvals: Mapping<(H160, H160), bool>,
    }

    impl Erc1155 {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                balances: Mapping::default(),
                operator_approvals: Mapping::default(),
            }
        }

        /// Mints `amount` of token `id` to the caller.
        #[ink(message)]
        pub fn mint(&mut self, id: TokenId, amount: Balance) -> Result<()> {
            let caller = self.env().caller();
            self.mint_token(&caller, id, amount)?;
            Ok(())
        }

        /// Returns the balance of `owner` for token `id`.
        #[ink(message)]
        pub fn balance_of(&self, owner: H160, id: TokenId) -> Balance {
            self.balances.get((owner, id)).unwrap_or(0)
        }

        /// Returns balances for a list of owners and ids.
        #[ink(message)]
        pub fn balance_of_batch(
            &self,
            owners: Vec<H160>,
            ids: Vec<TokenId>,
        ) -> Result<Vec<Balance>> {
            if owners.len() != ids.len() {
                return Err(Error::BatchSizeMismatch);
            }

            let mut output = Vec::new();
            for i in 0..owners.len() {
                output.push(self.balance_of(owners[i], ids[i]));
            }
            Ok(output)
        }

        /// Enables or disables an operator to manage all of caller's tokens.
        #[ink(message)]
        pub fn set_approval_for_all(&mut self, operator: H160, approved: bool) -> Result<()> {
            let caller = self.env().caller();
            self.operator_approvals.insert((caller, operator), &approved);
            
            self.env().emit_event(ApprovalForAll {
                owner: caller,
                operator,
                approved,
            });
            Ok(())
        }

        /// Checks if an operator is approved.
        #[ink(message)]
        pub fn is_approved_for_all(&self, owner: H160, operator: H160) -> bool {
            self.operator_approvals.get((owner, operator)).unwrap_or(false)
        }

        /// Transfers `amount` of token `id` from `from` to `to`.
        #[ink(message)]
        pub fn safe_transfer_from(
            &mut self,
            from: H160,
            to: H160,
            id: TokenId,
            amount: Balance,
        ) -> Result<()> {
            let caller = self.env().caller();

            // Auth check: Caller must be Owner OR Approved Operator
            if caller != from && !self.is_approved_for_all(from, caller) {
                return Err(Error::NotApproved);
            }

            self.transfer_token_from(&from, &to, id, amount)?;

            self.env().emit_event(TransferSingle {
                operator: caller,
                from: Some(from),
                to: Some(to),
                id,
                value: amount,
            });

            Ok(())
        }

        /// Transfers multiple token types in a single transaction (Batch).
        #[ink(message)]
        pub fn safe_batch_transfer_from(
            &mut self,
            from: H160,
            to: H160,
            ids: Vec<TokenId>,
            amounts: Vec<Balance>,
        ) -> Result<()> {
            let caller = self.env().caller();

            if caller != from && !self.is_approved_for_all(from, caller) {
                return Err(Error::NotApproved);
            }
            if ids.len() != amounts.len() {
                return Err(Error::BatchSizeMismatch);
            }

            // Loop through all tokens and transfer them one by one
            for i in 0..ids.len() {
                self.transfer_token_from(&from, &to, ids[i], amounts[i])?;
            }

            self.env().emit_event(TransferBatch {
                operator: caller,
                from: Some(from),
                to: Some(to),
                ids,
                values: amounts,
            });

            Ok(())
        }

        // --- Helpers ---

        fn mint_token(&mut self, to: &H160, id: TokenId, amount: Balance) -> Result<()> {
            let balance = self.balance_of(*to, id);
            self.balances.insert((*to, id), &(balance + amount));
            
            self.env().emit_event(TransferSingle {
                operator: self.env().caller(),
                from: None,
                to: Some(*to),
                id,
                value: amount,
            });
            Ok(())
        }

        fn transfer_token_from(
            &mut self,
            from: &H160,
            to: &H160,
            id: TokenId,
            amount: Balance,
        ) -> Result<()> {
            let from_balance = self.balance_of(*from, id);
            
            if from_balance < amount {
                return Err(Error::InsufficientBalance);
            }

            // Decrease Sender
            self.balances.insert((*from, id), &(from_balance - amount));
            
            // Increase Receiver
            let to_balance = self.balance_of(*to, id);
            self.balances.insert((*to, id), &(to_balance + amount));

            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn minting_works() {
            let mut contract = Erc1155::new();
            let alice = H160::from([0x01; 20]);
            
            // Default caller mints 100 units of Token ID 1
            assert_eq!(contract.mint(1, 100), Ok(()));
            
            // Check balance
            assert_eq!(contract.balance_of(H160::default(), 1), 100);
            // Check Token ID 2 is still 0
            assert_eq!(contract.balance_of(H160::default(), 2), 0);
        }

        #[ink::test]
        fn batch_transfer_works() {
            let mut contract = Erc1155::new();
            let bob = H160::from([0x02; 20]);
            
            // Mint 100 of ID 1, and 50 of ID 2
            let _ = contract.mint(1, 100);
            let _ = contract.mint(2, 50);

            // Send 10 of ID 1 AND 5 of ID 2 to Bob in one go
            assert_eq!(contract.safe_batch_transfer_from(
                H160::default(),
                bob,
                vec![1, 2],
                vec![10, 5]
            ), Ok(()));

            // Check Bob's balances
            assert_eq!(contract.balance_of(bob, 1), 10);
            assert_eq!(contract.balance_of(bob, 2), 5);
        }
    }
}
