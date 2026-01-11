use openbrush::traits::{ AccountId, Balance, Storage };


#[derive(Debug, Clone, Copy)]
#[openbrush::storage_item(PAIR_STORAGE_LOCATION)]
pub struct PairStorage {
    pub minimum_liquidity: Balance,
    pub factory: AccountId,
    pub token_0: AccountId,
    pub token_1: AccountId,
    pub reserve_0: Balance,
    pub reserve_1: Balance,
    pub block_timestamp_last: Balance,
    pub price0_cumulative_last: Balance,
    pub price1_cumulative_last: Balance,
    pub k_last: Balance,
}


impl Default for PairStorage {
    fn default() -> Self {
        Self { 
            minimum_liquidity: Balance::from(1000u32),
            factory: AccountId::from([0u8; 32]),
            token_0: AccountId::from([0u8; 32]),
            token_1: AccountId::from([0u8; 32]),
            reserve_0: Balance::default(),
            reserve_1: Balance::default(),
            block_timestamp_last: Balance::default(),
            price0_cumulative_last: Balance::default(),
            price1_cumulative_last: Balance::default(),
            k_last: Balance::default(),
        }
    }
}




// implementing utils functions for accessing storage (this makes life abit easier)
pub fn get_minimum_liquidity<T>(instance: &T) -> Balance where T: Storage<PairStorage> {
    instance.data::<PairStorage>().minimum_liquidity
}

pub fn get_factory<T>(instance: &T) -> AccountId where T: Storage<PairStorage> {
    instance.data::<PairStorage>().factory
}

pub fn set_factory<T>(instance: &mut T, factory: AccountId) where T: Storage<PairStorage> {
    instance.data::<PairStorage>().factory = factory;
}

pub fn get_token_0<T>(instance: &T) -> AccountId where T: Storage<PairStorage> {
    instance.data::<PairStorage>().token_0
}

pub fn set_token_0<T>(instance: &mut T, token_0: AccountId) where T: Storage<PairStorage> {
    instance.data::<PairStorage>().token_0 = token_0;
}

pub fn get_token_1<T>(instance: &T) -> AccountId where T: Storage<PairStorage> {
    instance.data::<PairStorage>().token_1
}

pub fn set_token_1<T>(instance: &mut T, token_1: AccountId) where T: Storage<PairStorage> {
    instance.data::<PairStorage>().token_1 = token_1;
}

pub fn get_reserve_0<T>(instance: &T) -> Balance where T: Storage<PairStorage> {
    instance.data::<PairStorage>().reserve_0
}

pub fn set_reserve_0<T>(instance: &mut T, reserve_0: Balance) where T: Storage<PairStorage> {
    instance.data::<PairStorage>().reserve_0 = reserve_0;
}

pub fn get_reserve_1<T>(instance: &T) -> Balance where T: Storage<PairStorage> {
    instance.data::<PairStorage>().reserve_1
}

pub fn set_reserve_1<T>(instance: &mut T, reserve_1: Balance) where T: Storage<PairStorage> {
    instance.data::<PairStorage>().reserve_1 = reserve_1;
}

pub fn get_block_timestamp_last<T>(instance: &T) -> Balance where T: Storage<PairStorage> {
    instance.data::<PairStorage>().block_timestamp_last
}

pub fn set_block_timestamp_last<T>(instance: &mut T, block_timestamp_last: Balance) where T: Storage<PairStorage> {
    instance.data::<PairStorage>().block_timestamp_last = block_timestamp_last;
}

pub fn get_price0_cumulative_last<T>(instance: &T) -> Balance where T: Storage<PairStorage> {
    instance.data::<PairStorage>().price0_cumulative_last
}

pub fn set_price0_cumulative_last<T>(instance: &mut T, price0_cumulative_last: Balance) where T: Storage<PairStorage> {
    instance.data::<PairStorage>().price0_cumulative_last = price0_cumulative_last;
}

pub fn get_price1_cumulative_last<T>(instance: &T) -> Balance where T: Storage<PairStorage> {
    instance.data::<PairStorage>().price1_cumulative_last
}

pub fn set_price1_cumulative_last<T>(instance: &mut T, price1_cumulative_last: Balance) where T: Storage<PairStorage> {
    instance.data::<PairStorage>().price1_cumulative_last = price1_cumulative_last;
}

pub fn get_k_last<T>(instance: &T) -> Balance where T: Storage<PairStorage> {
    instance.data::<PairStorage>().k_last
}

pub fn set_k_last<T>(instance: &mut T, k_last: Balance) where T: Storage<PairStorage> {
    instance.data::<PairStorage>().k_last = k_last;
}

