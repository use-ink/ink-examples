use ink::storage::Mapping;
use openbrush::traits::AccountId;
use ink::prelude::vec::Vec;
use openbrush::traits::Balance;



#[derive(Debug)]
#[openbrush::storage_item(STAKING_REWARD_STORAGE_LOCATION)]
pub struct StakingStorage {
    pub admin: AccountId,
    pub staked_token: AccountId,
    pub reward_token: AccountId,
    pub period_to_finish: Balance,
    pub reward_rate: Balance,
    pub reward_duration: Balance,
    pub last_updated_time: Balance,
    pub reward_per_token_stored: Balance,
    pub user_reward_per_token: Mapping<AccountId, Balance>,
    pub rewards: Mapping<AccountId, Balance>,
    pub total_supply: Balance,
    pub balances: Mapping<AccountId, Balance>,
}



impl Default for StakingStorage {
    fn default() -> Self {
        Self {
            admin: AccountId::from([0x0; 32]),
            staked_token: AccountId::from([0x0; 32]),
            reward_token: AccountId::from([0x0; 32]),
            period_to_finish: Default::default(),
            reward_rate: Default::default(),
            reward_duration: Default::default(),
            last_updated_time: Default::default(),
            reward_per_token_stored: Default::default(),
            user_reward_per_token: Default::default(),
            rewards: Default::default(),
            total_supply: Default::default(),
            balances: Default::default(),
        }
    }
}


