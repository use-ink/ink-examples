#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod erc20 {
    use ink::storage::Mapping;
    use ink::primitives::H160;

    /// Custom error types for the contract.
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InsufficientBalance,
        InsufficientAllowance,
    }

    /// Result type returning either success or a custom Error.
    pub type Result<T> = core::result::Result<T, Error>;

    /// Event emitted when tokens are transferred.
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<H160>,
        #[ink(topic)]
        to: Option<H160>,
        value: Balance,
    }

    /// Event emitted when an approval is created.
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: H160,
        #[ink(topic)]
        spender: H160,
        value: Balance,
    }

    #[ink(storage)]
    pub struct Erc20 {
        /// Total token supply in circulation.
        total_supply: Balance,
        /// Mapping from User Address (H160) to their Balance.
        balances: Mapping<H160, Balance>,
        /// Mapping from (Owner, Spender) to the Allowed Amount.
        allowances: Mapping<(H160, H160), Balance>,
    }

    impl Erc20 {
        /// Constructor: Initializes the contract with a total supply.
        /// The entire supply is initially assigned to the caller.
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut balances = Mapping::default();
            let caller = Self::env().caller();
            
            balances.insert(caller, &total_supply);

            Self::env().emit_event(Transfer {
                from: None,
                to: Some(caller),
                value: total_supply,
            });

            Self {
                total_supply,
                balances,
                allowances: Mapping::default(),
            }
        }

        /// Returns the total token supply.
        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        /// Returns the balance of the specified account.
        #[ink(message)]
        pub fn balance_of(&self, owner: H160) -> Balance {
            self.balances.get(owner).unwrap_or(0)
        }

        /// Transfers tokens from the caller to the recipient.
        #[ink(message)]
        pub fn transfer(&mut self, to: H160, value: Balance) -> Result<()> {
            let from = self.env().caller();
            self.transfer_from_to(&from, &to, value)
        }

        /// Internal helper to execute valid transfers and emit events.
        fn transfer_from_to(
            &mut self,
            from: &H160,
            to: &H160,
            value: Balance,
        ) -> Result<()> {
            let from_balance = self.balance_of(*from);
            
            if from_balance < value {
                return Err(Error::InsufficientBalance);
            }

            // Update balances
            self.balances.insert(from, &(from_balance - value));
            let to_balance = self.balance_of(*to);
            self.balances.insert(to, &(to_balance + value));

            self.env().emit_event(Transfer {
                from: Some(*from),
                to: Some(*to),
                value,
            });

            Ok(())
        }

        /// Approves a spender to transfer tokens on behalf of the caller.
        #[ink(message)]
        pub fn approve(&mut self, spender: H160, value: Balance) -> Result<()> {
            let owner = self.env().caller();
            self.allowances.insert((owner, spender), &value);
            
            self.env().emit_event(Approval {
                owner,
                spender,
                value,
            });
            
            Ok(())
        }

        /// Returns the remaining number of tokens that `spender` will be
        /// allowed to spend on behalf of `owner`.
        #[ink(message)]
        pub fn allowance(&self, owner: H160, spender: H160) -> Balance {
            self.allowances.get((owner, spender)).unwrap_or(0)
        }

        /// Transfers tokens from `from` to `to` using the allowance mechanism.
        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            from: H160,
            to: H160,
            value: Balance,
        ) -> Result<()> {
            let caller = self.env().caller();
            let allowance = self.allowance(from, caller);

            if allowance < value {
                return Err(Error::InsufficientAllowance);
            }

            self.allowances.insert((from, caller), &(allowance - value));
            self.transfer_from_to(&from, &to, value)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn new_works() {
            let contract = Erc20::new(1000);
            assert_eq!(contract.total_supply(), 1000);
        }

        #[ink::test]
        fn transfer_works() {
            let mut contract = Erc20::new(1000);
            let bob = H160::from([0x01; 20]); 
            
            assert_eq!(contract.transfer(bob, 100), Ok(()));
            assert_eq!(contract.balance_of(bob), 100);
        }

        #[ink::test]
        fn transfer_fails_if_insufficient() {
            let mut contract = Erc20::new(10);
            let bob = H160::from([0x01; 20]);
            assert_eq!(contract.transfer(bob, 20), Err(Error::InsufficientBalance));
        }
    }
}
