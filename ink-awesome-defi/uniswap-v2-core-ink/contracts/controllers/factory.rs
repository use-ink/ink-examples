
use openbrush::traits::AccountId;

use crate::providers::common::errors::UniswapV2Errors;



#[openbrush::wrapper]
pub type FactoryRef = dyn FactoryController;

#[openbrush::trait_definition]
pub trait FactoryController {
    // this function would set the address that receives the protocol fee
    #[ink(message)]
    fn set_fee_to(&mut self, _fee_to: AccountId) -> Result<(), UniswapV2Errors>;
    
    // this function is used the updated the address that can call the set_fee_to function
    #[ink(message)]
    fn set_fee_to_setter(&mut self, _fee_to_setter: AccountId) -> Result<(), UniswapV2Errors>;

    // this function is used to set the pair code hash (this is handy for upgrades)
    #[ink(message)]
    fn set_pair_code_hash(&mut self, _pair_code_hash: [u8; 32]) -> Result<(), UniswapV2Errors>;
 
}