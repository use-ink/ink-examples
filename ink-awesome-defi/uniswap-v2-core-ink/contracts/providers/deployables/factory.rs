use openbrush::{traits::{Storage, AccountId}, contracts::ownable::OwnableError};
use crate::providers::{data::factory::FactoryStorage, common::errors::UniswapV2Errors}; 


pub trait FactoryImpl:  
    Storage<FactoryStorage> 
{
    fn set_fee_to(&mut self, _fee_to: AccountId) -> Result<(), UniswapV2Errors> {
        let state = self.data::<FactoryStorage>();

        if state.fee_to_setter.unwrap() != Self::env().caller() {
            return Err(UniswapV2Errors::OwnableError(OwnableError::CallerIsNotOwner))
        }
        self.data::<FactoryStorage>().fee_to_setter = Some(_fee_to);
        Ok(())
    }

    fn set_fee_to_setter(&mut self, _fee_to_setter: AccountId) -> Result<(), UniswapV2Errors> {
        let state = self.data::<FactoryStorage>();

        if state.fee_to_setter.unwrap() != Self::env().caller() {
            return Err(UniswapV2Errors::OwnableError(OwnableError::CallerIsNotOwner))
        }
        self.data::<FactoryStorage>().fee_to_setter = Some(_fee_to_setter);
        Ok(())
    }

    fn set_pair_code_hash(&mut self, _pair_code_hash: [u8; 32]) -> Result<(), UniswapV2Errors> {
        let state = self.data::<FactoryStorage>();

        if state.fee_to_setter.unwrap() != Self::env().caller() {
            return Err(UniswapV2Errors::OwnableError(OwnableError::CallerIsNotOwner))
        }
        self.data::<FactoryStorage>().pair_code_hash = _pair_code_hash;
        Ok(())
    }
}