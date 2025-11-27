#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod SimpleToken {

    use ink::storage::Mapping;


    #[ink(storage)]
    pub struct SimpleToken {
        balances: Mapping<ink::Address, Balance>
    }

    impl SimpleToken {
        #[ink(constructor)]
        pub fn new(initial_supply: Balance) -> Self {
            let mut new = Mapping::new();
            new.insert(ink::env::caller(), &initial_supply); 
            Self { balances: new  }
        }

        #[ink(message)]
        pub fn balance_of(&self, owner: ink::Address) -> Balance {
            self.balances.get(&owner).unwrap_or(0)
        }
    }
}
