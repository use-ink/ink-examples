
use openbrush::traits::AccountId;
use openbrush::traits::Balance;
use crate::providers::common::errors::StakingRewardsErrors;



#[openbrush::wrapper]
pub type StakingRewardRef = dyn StakingRewardController;

#[openbrush::trait_definition]
pub trait StakingRewardController {
    #[ink(message)]
    fn get_reward_token(&self) -> AccountId;

    #[ink(message)]
    fn get_staked_token(&self) -> AccountId;

    #[ink(message)]
    fn get_reward_rate(&self) -> Balance;

    #[ink(message)]
    fn get_period_finish(&self) -> Balance;

    #[ink(message)]
    fn get_reward_duration(&self) -> Balance;

    #[ink(message)]
    fn get_last_update_time(&self) -> Balance;

    #[ink(message)]
    fn get_reward_per_token_stored(&self) -> Balance;

    #[ink(message)]
    fn get_user_reward_per_token_paid(&self, _account: AccountId) -> Balance;

    #[ink(message)]
    fn get_rewards(&self, _account: AccountId) -> Balance;

    #[ink(message)]
    fn total_supply(&self) -> Balance;

    #[ink(message)]
    fn balance_of(&self, _account: AccountId) -> Balance;

    #[ink(message)]
    fn last_time_reward_applicable(&self) -> Balance;

    #[ink(message)]
    fn reward_per_token(&self) -> Balance;

    #[ink(message)]
    fn earned(&self, _account: AccountId) -> Balance;

    #[ink(message)]
    fn get_reward_for_duration(&self) -> Balance;



    #[ink(message)]
    fn stake(&mut self, _amount: Balance) -> Result<(), StakingRewardsErrors>;

    #[ink(message)]
    fn withdraw(&mut self, _amount: Balance) -> Result<(), StakingRewardsErrors>;

    #[ink(message)]
    fn get_reward(&mut self) -> Result<(), StakingRewardsErrors>;

    #[ink(message)]
    fn exit(&mut self) -> Result<(), StakingRewardsErrors>;

    #[ink(message)]
    fn notify_reward_amount(&mut self, _reward: Balance) -> Result<(), StakingRewardsErrors>;

    #[ink(message)]
    fn recover_erc20(&mut self, _token: AccountId, _amount: Balance) -> Result<(), StakingRewardsErrors>;

    #[ink(message)]
    fn set_reward_duration(&mut self, _duration: Balance) -> Result<(), StakingRewardsErrors>;
}