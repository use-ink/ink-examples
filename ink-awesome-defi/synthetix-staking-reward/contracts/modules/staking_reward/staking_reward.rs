#![cfg_attr(not(feature = "std"), no_std, no_main)]



#[ink::contract]
mod staking_reward {
    use ink::prelude::vec::Vec;
    use global::providers::common::errors::StakingRewardsErrors;
    use global::providers::data::staking_reward::StakingStorage;
    use global::controllers::staking_reward::stakingrewardcontroller_external::StakingRewardController;
    use global::providers::deployables::staking_reward::StakingRewardImpl;
    use openbrush::contracts::ownable;
    use openbrush::traits::Storage;
    use global::controllers::staking_reward::stakingrewardcontroller_external;
   


    // ======================================
    // Storage
    // ======================================
    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct StakingReward {
        #[storage_field]
        pub staking_state: StakingStorage,
        #[storage_field]
        ownable: ownable::Data,
    }



    // ======================================
    // Events
    // ======================================
    #[ink(event)]
    pub struct Staked {
        #[ink(topic)]
        caller:AccountId,
        amount: Balance,
    }

    #[ink(event)]
    pub struct Withdraw {
        #[ink(topic)]
        caller:AccountId,
        amount: Balance,
    }

    #[ink(event)]
    pub struct RewardPaid {
        #[ink(topic)]
        caller:AccountId,
        reward: Balance,
    }

    #[ink(event)]
    pub struct RewardNotified {
        reward: Balance,
    }

    #[ink(event)]
    pub struct DurationUpdate {
        duration: Balance,
    }




    impl  StakingRewardImpl for StakingReward {}


    impl StakingRewardController for StakingReward {
        #[ink(message)]
        fn total_supply(&self) -> Balance {
            StakingRewardImpl::total_supply(self)
        }

        #[ink(message)]
        fn get_reward_token(&self) -> AccountId {
            StakingRewardImpl::get_reward_token(self)
        }

        #[ink(message)]
        fn get_staked_token(&self) -> AccountId {
            StakingRewardImpl::get_staked_token(self)
        }

        #[ink(message)]
        fn get_reward_rate(&self) -> Balance {
            StakingRewardImpl::get_reward_rate(self)
        }

        #[ink(message)]
        fn get_period_finish(&self) -> Balance {
            StakingRewardImpl::get_period_finish(self)
        }

        #[ink(message)]
        fn get_reward_duration(&self) -> Balance {
            StakingRewardImpl::get_reward_duration(self)
        }

        #[ink(message)]
        fn get_last_update_time(&self) -> Balance {
            StakingRewardImpl::get_last_update_time(self)
        }

        #[ink(message)]
        fn get_reward_per_token_stored(&self) -> Balance {
            StakingRewardImpl::get_reward_per_token_stored(self)
        }

        #[ink(message)]
        fn get_user_reward_per_token_paid(&self, _account: AccountId) -> Balance {
            StakingRewardImpl::get_user_reward_per_token_paid(self, _account)
        }

        #[ink(message)]
        fn get_rewards(&self, _account: AccountId) -> Balance {
            StakingRewardImpl::get_rewards(self, _account)
        }

        #[ink(message)]
        fn balance_of(&self, _account: AccountId) -> Balance {
            StakingRewardImpl::balance_of(self, _account)
        }

        #[ink(message)]
        fn last_time_reward_applicable(&self) -> Balance {
            StakingRewardImpl::last_time_reward_applicable(self)
        }

        #[ink(message)]
        fn reward_per_token(&self) -> Balance {
            StakingRewardImpl::reward_per_token(self)
        }

        #[ink(message)]
        fn earned(&self, _account: AccountId) -> Balance {
            StakingRewardImpl::earned(self, _account)
        }

        #[ink(message)]
        fn get_reward_for_duration(&self) -> Balance {
            StakingRewardImpl::get_reward_for_duration(self)
        }





        #[ink(message)]
        fn stake(&mut self, _amount: Balance) -> Result<(), StakingRewardsErrors> {
            StakingRewardImpl::stake(self, _amount)
        }

        #[ink(message)]
        fn withdraw(&mut self, _amount: Balance) -> Result<(), StakingRewardsErrors> {
            StakingRewardImpl::withdraw(self, _amount)
        }

        #[ink(message)]
        fn get_reward(&mut self) -> Result<(), StakingRewardsErrors> {
            StakingRewardImpl::get_reward(self)
        }

        #[ink(message)]
        fn exit(&mut self) -> Result<(), StakingRewardsErrors> {
            StakingRewardImpl::exit(self)
        }

        #[ink(message)]
        fn notify_reward_amount(&mut self, _reward: Balance) -> Result<(), StakingRewardsErrors> {
            StakingRewardImpl::notify_reward_amount(self, _reward)
        }

        #[ink(message)]
        fn recover_erc20(&mut self, _token: AccountId, _amount: Balance) -> Result<(), StakingRewardsErrors> {
            StakingRewardImpl::recover_erc20(self, _token, _amount)
        }

        #[ink(message)]
        fn set_reward_duration(&mut self, _duration: Balance) -> Result<(), StakingRewardsErrors> {
            StakingRewardImpl::set_reward_duration(self, _duration)
        }
    }

    
    
    
    
    
    
    
    
    
    
    
    impl StakingReward {
      
        #[ink(constructor)]
        pub fn new(_owner: AccountId, _reward_token: AccountId, _staked_token: AccountId) -> Self {
            let mut instance = Self::default();


            instance.staking_state.admin = _owner;
            instance.staking_state.staked_token = _staked_token;
            instance.staking_state.reward_token = _reward_token;


            instance
        }

    }

}