use core::cell::RefCell;
use openbrush::{traits::{Storage, AccountId}, contracts::ownable::OwnableError};
use crate::providers::{data::staking_reward::StakingStorage, common::errors::StakingRewardsErrors}; 
use openbrush::traits::Balance;
use openbrush::{contracts::{psp22::{psp22, extensions::metadata::PSP22MetadataImpl}, traits::psp22::PSP22Ref}};
use ink::prelude::vec::Vec;
use openbrush::modifier_definition;





pub trait StakingRewardImpl:  
    Storage<StakingStorage> 
{
    // ======================================
    // Mutating functions
    // ======================================
    fn stake(&mut self, _amount: Balance) -> Result<(), StakingRewardsErrors> {
        let state = self.data::<StakingStorage>();
        let caller = Self::env().caller();
        if _amount <= 0 {
            return Err(StakingRewardsErrors::InvalidAmount);
        }

        state.total_supply += _amount;
        let current_balance = state.balances.get(&caller).unwrap_or(0u128);
        state.balances.insert(caller, &(current_balance + _amount));

        PSP22Ref::transfer_from(&state.staked_token, caller, Self::env().account_id(), _amount, Vec::<u8>::new()).map_err(|_| StakingRewardsErrors::TokenTransferFailed)?;
        Ok(())
    }

    fn withdraw(&mut self, _amount: Balance) -> Result<(), StakingRewardsErrors> {
        let state = self.data::<StakingStorage>();
        let caller = Self::env().caller();
        if _amount <= 0 {
            return Err(StakingRewardsErrors::InvalidAmount);
        }

        state.total_supply -= _amount;
        let current_balance = state.balances.get(&caller).unwrap_or(0u128);
        state.balances.insert(caller, &(current_balance - _amount));

        PSP22Ref::transfer(&state.staked_token, caller, _amount, Vec::<u8>::new()).map_err(|_| StakingRewardsErrors::TokenTransferFailed)?;
        Ok(())
    }

    fn get_reward(&mut self) -> Result<(), StakingRewardsErrors> {
        let state = self.data::<StakingStorage>();
        let caller = Self::env().caller();


        let reward = state.rewards.get(&caller).unwrap_or(0u128);

        if reward <= 0 {
            return Err(StakingRewardsErrors::InvalidAmount);
        }

        state.rewards.insert(caller, &0u128);
        PSP22Ref::transfer(&state.reward_token, caller, reward, Vec::<u8>::new()).map_err(|_| StakingRewardsErrors::TokenTransferFailed)?;
        Ok(())
    }

    fn exit(&mut self) -> Result<(), StakingRewardsErrors> {
        let state = self.data::<StakingStorage>();
        let caller = Self::env().caller();

        let amount = state.balances.get(&caller).unwrap_or(0u128);
        self.withdraw(amount)?;
        self.get_reward()?;
        Ok(())
    }






    fn notify_reward_amount(&mut self, _reward: Balance) -> Result<(), StakingRewardsErrors> {
        let state = self.data::<StakingStorage>();
        let caller = Self::env().caller();

        // remove this whe you plug in the modifer
        if caller != state.admin {
            return Err(StakingRewardsErrors::NotAdmin);
        }

        if _reward <= 0 {
            return Err(StakingRewardsErrors::InvalidAmount);
        }

        let block_timestamp = Self::env().block_timestamp() as u128;
        if block_timestamp >= state.period_to_finish {
            state.reward_rate = _reward / state.reward_duration;
        } else {
            let remaining = state.period_to_finish - block_timestamp;
            let leftover = remaining * state.reward_rate;
            state.reward_rate = (_reward + leftover) / state.reward_duration;
        }


        let balance = PSP22Ref::balance_of(&state.reward_token, Self::env().account_id());

        if state.reward_rate > balance / state.reward_duration {
            return Err(StakingRewardsErrors::ProviderRewardIsToLarge);
        }
        state.last_updated_time = block_timestamp;
        state.period_to_finish = block_timestamp + state.reward_duration;

        Ok(())
    }


    fn recover_erc20(&mut self, _token: AccountId, _amount: Balance) -> Result<(), StakingRewardsErrors> {
        let state = self.data::<StakingStorage>();
        let caller = Self::env().caller();

        // remove this whe you plug in the modifer
        if caller != state.admin {
            return Err(StakingRewardsErrors::NotAdmin);
        }

        if state.reward_token == _token {
            return Err(StakingRewardsErrors::CannotWithdrawRewardToken);
        }

        if state.staked_token == _token {
            return Err(StakingRewardsErrors::CannotWithdrawStakedToken);
        }

        if _amount <= 0 {
            return Err(StakingRewardsErrors::InvalidAmount);
        }

        PSP22Ref::transfer(&state.reward_token, caller, _amount, Vec::<u8>::new()).map_err(|_| StakingRewardsErrors::TokenTransferFailed)?;
        Ok(())
    }

    fn set_reward_duration(&mut self, _duration: Balance) -> Result<(), StakingRewardsErrors> {
        let state = self.data::<StakingStorage>();
        let caller = Self::env().caller();

        // remove this whe you plug in the modifer
        if caller != state.admin {
            return Err(StakingRewardsErrors::NotAdmin);
        }

        if _duration <= 0 {
            return Err(StakingRewardsErrors::InvalidAmount);
        }

        if (Self::env().block_timestamp() as u128) < state.period_to_finish {
            return Err(StakingRewardsErrors::StakingStillInProgress);
        }

        state.reward_duration = _duration;
        Ok(())
    }









    // ======================================
    // View functions
    // ======================================
    fn get_reward_token(&self) -> AccountId {
        self.data::<StakingStorage>().reward_token
    }

    fn get_staked_token(&self) -> AccountId {
        self.data::<StakingStorage>().staked_token
    }

    fn get_reward_rate(&self) -> Balance {
        self.data::<StakingStorage>().reward_rate
    }

    fn get_period_finish(&self) -> Balance {
        self.data::<StakingStorage>().period_to_finish
    }

    fn get_reward_duration(&self) -> Balance {
        self.data::<StakingStorage>().reward_duration
    }

    fn get_last_update_time(&self) -> Balance {
        self.data::<StakingStorage>().last_updated_time
    }

    fn get_reward_per_token_stored(&self) -> Balance {
        self.data::<StakingStorage>().reward_per_token_stored
    }

    fn get_user_reward_per_token_paid(&self, _account: AccountId) -> Balance {
        self.data::<StakingStorage>().user_reward_per_token.get(&_account).unwrap_or(0u128).clone()
    }

    fn get_rewards(&self, _account: AccountId) -> Balance {
        self.data::<StakingStorage>().rewards.get(&_account).unwrap_or(0u128).clone()
    }

    fn total_supply(&self) -> Balance {
        self.data::<StakingStorage>().total_supply
    }

    fn balance_of(&self, _account: AccountId) -> Balance {
        self.data::<StakingStorage>().balances.get(&_account).unwrap_or(0u128).clone()
    }

    fn last_time_reward_applicable(&self) -> Balance {
        let state = self.data::<StakingStorage>();
        let block_timestamp = Self::env().block_timestamp() as u128;
        block_timestamp.min(state.period_to_finish)
    }

    fn reward_per_token(&self) -> Balance {
        let state = self.data::<StakingStorage>();
        if state.total_supply == 0 {
            state.reward_per_token_stored
        } else {
            state.reward_per_token_stored + (self.last_time_reward_applicable() - state.last_updated_time) * state.reward_rate * 10u128.pow(18) / state.total_supply
        }
    }

    fn earned(&self, _account: AccountId) -> Balance {
        let state = self.data::<StakingStorage>();
        let user_reward_per_token_paid = state.user_reward_per_token.get(&_account).unwrap_or(0u128).clone();
        let reward_per_token = self.reward_per_token();
        let balance_of = self.balance_of(_account);
        let earned = balance_of * (reward_per_token - user_reward_per_token_paid) / 10u128.pow(18) + state.rewards.get(&_account).unwrap_or(0u128);
        
        earned
    }

    fn get_reward_for_duration(&self) -> Balance {
        let state = self.data::<StakingStorage>();
        state.reward_rate * state.reward_duration
    }

    fn get_caller(&self) -> AccountId {
        Self::env().caller()
    }
}





// ======================================
// Modifers
// ======================================
//TODO: move this logic into the trait... modifer_definition! macro does not accept abr data
// #[modifier_definition]
// pub fn update_reward<T, F, R, E>(instance: &mut T, body: F, is_address_zero: bool) -> Result<R, E>
//     where T: Storage<StakingStorage> + StakingRewardImpl, F: FnOnce(&mut T) -> Result<R, E>, E: From<StakingRewardsErrors>
// {

//     let instance_ = RefCell::new(instance);
//     let mut instance_1 = instance_.borrow_mut();
//     let state = instance_1.data::<StakingStorage>();
//     let reward_per_token = StakingRewardImpl::reward_per_token(instance);
//     let last_updated_time = StakingRewardImpl::last_time_reward_applicable(instance);

//     if is_address_zero {
//         let current_earned = StakingRewardImpl::earned(instance, instance.get_caller());
//         state.rewards.insert(instance.get_caller(), &current_earned);
//         state.user_reward_per_token.insert(instance.get_caller(), &reward_per_token);
//     }

//     body(instance)
// }
