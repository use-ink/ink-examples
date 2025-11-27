#![cfg_attr(not(feature = "std"), no_std, no_main)]

// Factory contract that can deploy instances of the SimpleToken contract
// It uses cross-contract calls to instantiate new token contracts and factory pattern

#[ink::contract]
mod TokenFactory {

    use ink::storage::StorageVec;
    use ink::prelude::vec::Vec;
    use SimpleToken::SimpleTokenRef;

    #[ink(storage)]
    pub struct TokenFactory {
        token_code_hash: ink::H256,
        children: StorageVec<ink::Address>,
    }

    impl TokenFactory {

        #[ink(constructor)]
        pub fn new(token_code_hash: ink::H256) -> Self {
            Self {
                token_code_hash,
                children: StorageVec::new(),
            }
        }

        // Creates a new instance of the SimpleToken contract with the initial parameter
        #[ink(message)]
        pub fn create_token(&mut self, initial_supply: Balance) {
            let token_instance = SimpleTokenRef::new(initial_supply) // Here happens the cross-contract call, we promise to deploy a new SimpleToken and we will store its address to our mapping "children", so the factory can track the deployed tokens
                .code_hash(self.token_code_hash) // We provide the code hash of the SimpleToken contract to instantiate it, ensuring that the correct contract code is used
                .salt_bytes(None) 
                .instantiate();
            let token_address = ink::ToAddr::to_addr(&token_instance);
            self.children.push(&token_address);
        }

        #[ink(message)]
        pub fn list_tokens(&self) -> Vec<ink::Address> {
            let mut tokens = Vec::new();
            for i in 0..self.children.len() {
                tokens.push(self.children.get(i).unwrap());
            }
            tokens
        }      
    }  
}
