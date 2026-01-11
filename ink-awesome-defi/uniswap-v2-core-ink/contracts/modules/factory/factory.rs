#![cfg_attr(not(feature = "std"), no_std, no_main)]



#[ink::contract]
mod factory {
    use ink::prelude::vec::Vec;
    use global::providers::common::errors::UniswapV2Errors;
    use global::providers::data::factory::FactoryStorage;
    // use global::controllers::factory::factorycontroller_external;
    // use global::controllers::factory::FactoryController;
    use global::controllers::factory::factorycontroller_external::FactoryController;
    use global::providers::deployables::factory::FactoryImpl;
    use openbrush::traits::Storage;
    use pair::pair::PairRef;
    use ink::ToAccountId;


   
    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Factory {
        #[storage_field]
        pub factory_state: FactoryStorage,

    }

    impl  FactoryImpl for Factory {}


    impl FactoryController for Factory {
        #[ink(message)]
        fn set_fee_to(&mut self, _fee_to: AccountId) -> Result<(), UniswapV2Errors> {
            FactoryImpl::set_fee_to(self, _fee_to)
        }

        #[ink(message)]
        fn set_fee_to_setter(&mut self, _fee_to_setter: AccountId) -> Result<(), UniswapV2Errors> {
            FactoryImpl::set_fee_to_setter(self, _fee_to_setter)
        }

        #[ink(message)]
        fn set_pair_code_hash(&mut self, _pair_code_hash: [u8; 32]) -> Result<(), UniswapV2Errors> {
            FactoryImpl::set_pair_code_hash(self, _pair_code_hash)
        }
    }

    impl Factory {
      
        #[ink(constructor)]
        pub fn new(pair_code_hash: [u8; 32]) -> Self {
            let mut instance = Self::default();

            instance.factory_state.fee_to_setter = Some(Self::env().caller());
            instance.factory_state.pair_code_hash = pair_code_hash;


            instance
        }




        #[ink(message)]
        pub fn create_pair(&mut self, token_a: AccountId, token_b: AccountId, salt_bytes: Vec<u8>) -> Result<AccountId, UniswapV2Errors> {
            let pair = self.factory_state.get_pair.get(&(token_a, token_b));
            if pair.is_some() {
                return Err(UniswapV2Errors::PairAlreadyExists);
            }

            let pair = self.factory_state.get_pair.get(&(token_b, token_a));
            if pair.is_some() {
                return Err(UniswapV2Errors::PairAlreadyExists);
            }

            let hash = self.factory_state.pair_code_hash;

            let pair = PairRef::new(token_a, token_b)
                .endowment(0)
                .code_hash(hash.into())
                .salt_bytes(&salt_bytes)
                .instantiate();

            let pair_addr = pair.to_account_id();

            self.factory_state.get_pair.insert(&(token_a, token_b), &pair_addr);
            self.factory_state.get_pair.insert(&(token_b, token_a), &pair_addr);

            self.factory_state.all_pairs.push(pair_addr);

            Ok(pair_addr)
        }
      
      
    }

}