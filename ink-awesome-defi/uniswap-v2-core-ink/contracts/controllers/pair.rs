use ink::primitives::AccountId;
use openbrush::traits::Balance;

use crate::providers::common::errors::UniswapV2Errors;
use openbrush::contracts::traits::psp22::*;


#[openbrush::wrapper]
pub type PairRef = dyn PairController + PSP22;

#[openbrush::trait_definition]
pub trait PairController {
    
    #[ink(message)]
    fn initialize(&mut self, _token0: AccountId, _token1: AccountId);

    #[ink(message)]
    fn mint(&mut self, _to: AccountId) -> Result<(), UniswapV2Errors>;

    #[ink(message)]
    fn burn(&mut self, _to: AccountId) -> Result<(), UniswapV2Errors>;

    #[ink(message)]
    fn swap(&mut self, _amount0: Balance, _amount1: Balance, _to: AccountId)  -> Result<(), UniswapV2Errors>;

    #[ink(message)]
    fn skim(&mut self, _to: AccountId)  -> Result<(), UniswapV2Errors>;
    
    #[ink(message)]
    fn sync(&mut self) -> Result<(), UniswapV2Errors>;



    #[ink(message)]
    fn token_0(&self) -> AccountId;

    #[ink(message)]
    fn token_1(&self) -> AccountId;

    #[ink(message)]
    fn factory(&self) -> AccountId;

    #[ink(message)]
    fn price0_cumulative_last(&self) -> u128;

    #[ink(message)]
    fn price1_cumulative_last(&self) -> u128;   

    #[ink(message)]
    fn get_reserves(&self) -> (u128, u128, u128);
}