use ink::storage::Mapping;
use openbrush::traits::AccountId;
use ink::prelude::vec::Vec;



#[derive(Debug)]
#[openbrush::storage_item(FACTORY_STORAGE_LOCATION)]
pub struct FactoryStorage {
    pub fee_to: Option<AccountId>,
    pub fee_to_setter: Option<AccountId>,
    pub get_pair: Mapping<(AccountId, AccountId), AccountId>,
    pub all_pairs: Vec<AccountId>,
    pub pair_code_hash: [u8; 32],
}



impl Default for FactoryStorage {
    fn default() -> Self {
        Self {
            fee_to: Default::default(),
            fee_to_setter: Default::default(),
            get_pair: Default::default(),
            all_pairs: Default::default(),
            pair_code_hash: Default::default(),
        }
    }
}


