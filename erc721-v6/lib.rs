#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod erc721 {
    use ink::storage::Mapping;
    use ink::primitives::H160;

    /// We define the Token ID as a simple 32-bit number.
    pub type TokenId = u32;

    /// Custom Error types.
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        NotOwner,
        NotApproved,
        TokenExists,
        TokenNotFound,
        CannotFetchValue,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    /// Event: Transfer of ownership.
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<H160>,
        #[ink(topic)]
        to: Option<H160>,
        #[ink(topic)]
        id: TokenId,
    }

    /// Event: Approval of a single token.
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: H160,
        #[ink(topic)]
        spender: H160,
        #[ink(topic)]
        id: TokenId,
    }

    /// Event: Approval for an operator to manage ALL tokens of an owner.
    #[ink(event)]
    pub struct ApprovalForAll {
        #[ink(topic)]
        owner: H160,
        #[ink(topic)]
        operator: H160,
        approved: bool,
    }

    #[ink(storage)]
    pub struct Erc721 {
        /// Mapping from Token ID to Owner Address.
        token_owner: Mapping<TokenId, H160>,
        /// Mapping from Token ID to Approved Spender.
        token_approvals: Mapping<TokenId, H160>,
        /// Mapping from Owner to how many tokens they own.
        owned_tokens_count: Mapping<H160, u32>,
        /// Mapping from (Owner, Operator) to boolean (Approved or Not).
        operator_approvals: Mapping<(H160, H160), bool>,
    }

    impl Erc721 {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                token_owner: Mapping::default(),
                token_approvals: Mapping::default(),
                owned_tokens_count: Mapping::default(),
                operator_approvals: Mapping::default(),
            }
        }

        /// Returns the number of tokens owned by `owner`.
        #[ink(message)]
        pub fn balance_of(&self, owner: H160) -> u32 {
            self.owned_tokens_count.get(owner).unwrap_or(0)
        }

        /// Returns the owner of the specified token ID.
        #[ink(message)]
        pub fn owner_of(&self, id: TokenId) -> Option<H160> {
            self.token_owner.get(id)
        }

        /// Mints a new token. (Creates it).
        #[ink(message)]
        pub fn mint(&mut self, id: TokenId) -> Result<()> {
            if self.token_owner.contains(id) {
                return Err(Error::TokenExists);
            }

            let caller = self.env().caller();
            self.add_token_to(&caller, id)?;

            self.env().emit_event(Transfer {
                from: None,
                to: Some(caller),
                id,
            });

            Ok(())
        }

        /// Transfers token `id` to `to`.
        #[ink(message)]
        pub fn transfer(&mut self, to: H160, id: TokenId) -> Result<()> {
            let caller = self.env().caller();
            self.transfer_token_from(&caller, &to, id)
        }

        /// Transfers token `id` from `from` to `to`.
        /// Requires approval.
        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            from: H160,
            to: H160,
            id: TokenId,
        ) -> Result<()> {
            let caller = self.env().caller();
            
            // Check if caller is owner, approved for this token, or approved for all
            let owner = self.owner_of(id).ok_or(Error::TokenNotFound)?;
            if owner != from {
                return Err(Error::NotOwner);
            }

            let approved_address = self.get_approved(id);
            let is_approved = approved_address == Some(caller);
            let is_operator = self.is_approved_for_all(from, caller);

            if !is_approved && !is_operator && caller != from {
                return Err(Error::NotApproved);
            }

            self.transfer_token_from(&from, &to, id)
        }

        /// Approves `to` to spend token `id`.
        #[ink(message)]
        pub fn approve(&mut self, to: H160, id: TokenId) -> Result<()> {
            let caller = self.env().caller();
            let owner = self.owner_of(id).ok_or(Error::TokenNotFound)?;

            if owner != caller {
                return Err(Error::NotOwner);
            }

            // Set the approval
            self.token_approvals.insert(id, &to);
            
            self.env().emit_event(Approval {
                owner,
                spender: to,
                id,
            });

            Ok(())
        }

        /// Returns the approved address for a token ID.
        #[ink(message)]
        pub fn get_approved(&self, id: TokenId) -> Option<H160> {
            self.token_approvals.get(id)
        }

        /// Approves `operator` to spend ALL tokens owned by caller.
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

        /// Checks if `operator` is approved for all tokens of `owner`.
        #[ink(message)]
        pub fn is_approved_for_all(&self, owner: H160, operator: H160) -> bool {
            self.operator_approvals.get((owner, operator)).unwrap_or(false)
        }

        // --- Helper Functions ---

        fn add_token_to(&mut self, to: &H160, id: TokenId) -> Result<()> {
            if self.token_owner.contains(id) {
                return Err(Error::TokenExists);
            }

            self.token_owner.insert(id, to);
            
            let count = self.owned_tokens_count.get(to).unwrap_or(0);
            self.owned_tokens_count.insert(to, &(count + 1));
            
            Ok(())
        }

        fn remove_token_from(&mut self, from: &H160, id: TokenId) -> Result<()> {
            if !self.token_owner.contains(id) {
                return Err(Error::TokenNotFound);
            }

            self.token_owner.remove(id);
            
            let count = self.owned_tokens_count.get(from).ok_or(Error::CannotFetchValue)?;
            self.owned_tokens_count.insert(from, &(count - 1));
            
            Ok(())
        }

        fn transfer_token_from(
            &mut self,
            from: &H160,
            to: &H160,
            id: TokenId,
        ) -> Result<()> {
            let owner = self.owner_of(id).ok_or(Error::TokenNotFound)?;
            if owner != *from {
                return Err(Error::NotOwner);
            }

            // Clear approval for this specific token
            self.token_approvals.remove(id);

            self.remove_token_from(from, id)?;
            self.add_token_to(to, id)?;

            self.env().emit_event(Transfer {
                from: Some(*from),
                to: Some(*to),
                id,
            });

            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn mint_works() {
            let mut contract = Erc721::new();
            let alice = H160::from([0x01; 20]);
            
            // Mocking call from Alice (in unit tests default caller is usually same, 
            // but for clarity we assume Alice calls this implicitly in this mock env)
            
            assert_eq!(contract.mint(1), Ok(()));
            assert_eq!(contract.owner_of(1), Some(H160::default()));
        }

        #[ink::test]
        fn transfer_works() {
            let mut contract = Erc721::new();
            // Default account mints ID 1
            assert_eq!(contract.mint(1), Ok(()));
            
            let bob = H160::from([0x02; 20]);
            
            // Default account transfers ID 1 to Bob
            assert_eq!(contract.transfer(bob, 1), Ok(()));
            
            // Check Bob is owner
            assert_eq!(contract.owner_of(1), Some(bob));
            // Check default account balance decreased
            assert_eq!(contract.balance_of(H160::default()), 0);
        }
    }
}
