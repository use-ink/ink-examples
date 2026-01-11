
use ink::prelude::vec::Vec;
use ink::primitives::AccountId;
use openbrush::{traits::{Storage, Balance}, contracts::{psp22::{psp22, extensions::metadata::PSP22MetadataImpl}, traits::psp22::PSP22Ref}};
use crate::providers::{data::pair::{PairStorage, get_token_0, get_token_1, get_factory, get_price0_cumulative_last, get_price1_cumulative_last}, common::errors::UniswapV2Errors};
use scale::CompactAs; 
use sp_arithmetic::FixedU128; 


pub trait PairImpl:  
    Storage<PairStorage> +
    Storage<psp22::Data> +
    PSP22MetadataImpl +
    psp22::Internal + 
{
    // public varible get functions
    fn token_0(&self) -> AccountId {
        get_token_0(self)
    }

    fn token_1(&self) -> AccountId {
        get_token_1(self)
    }

    fn factory(&self) -> AccountId {
        get_factory(self)
    }

    fn price0_cumulative_last(&self) -> u128 {
        get_price0_cumulative_last(self)
    }

    fn price1_cumulative_last(&self) -> u128 {
        get_price1_cumulative_last(self)
    }

    fn get_reserves(&self) -> (u128, u128, u128) {
        let state = self.data::<PairStorage>();
        (state.reserve_0, state.reserve_1, state.block_timestamp_last)
    }







    
    fn initialize(&mut self,  _token0: AccountId, _token1: AccountId) {
        let state = self.data::<PairStorage>();
        state.token_0 = _token0;
        state.token_1 = _token1;
    }

    fn mint(&mut self, _to: AccountId) -> Result<(), UniswapV2Errors> {
        let state = *self.data::<PairStorage>();
        let this = Self::env().account_id();
        let (reserve_0, reserve_1, minimum_liquidity) = (state.reserve_0, state.reserve_1, state.minimum_liquidity);
        let balance0 = PSP22Ref::balance_of(&state.token_0, this);
        let balance1 = PSP22Ref::balance_of(&state.token_1, this);
        let amount0 = balance0 - reserve_0;
        let amount1 = balance1 - reserve_1;
        let liqudity: Balance;

        let fee_on = self.internal_mint_fee();


        if self._total_supply() == 0 {
            // this is the first liquidity is been added to the pool
            liqudity = internal_sqrt(amount0 * amount1) - minimum_liquidity;
            self._mint_to(AccountId::from([0u8; 32]), minimum_liquidity)?;
        } else {
            liqudity = (amount0 * self._total_supply() / reserve_0).min(amount1 * self._total_supply() / reserve_1);
        }

        if liqudity == 0 {
            return Err(UniswapV2Errors::InsufficientLiquidityMinted);
        }

        self._mint_to(_to, liqudity)?;

        self.internal_update(balance0, balance1)?;

        if fee_on.unwrap_or(false) {
            self.data::<PairStorage>().k_last = (self.data::<PairStorage>().reserve_0 as u128) * (self.data::<PairStorage>().reserve_1 as u128);
        }

        Ok(())
    }


    fn burn(&mut self, _to: AccountId) -> Result<(), UniswapV2Errors> {
        let state = *self.data::<PairStorage>();
        let this = Self::env().account_id();
        let (token_0, token_1) = (state.token_0, state.token_1);
        let mut balance0 = PSP22Ref::balance_of(&token_0, this);
        let mut balance1 = PSP22Ref::balance_of(&token_1, this);
        let liquidity = self._balance_of(&this);

        let fee_on = self.internal_mint_fee();
        let _total_supply = self._total_supply();

        let amount0 = liquidity * balance0 / _total_supply;
        let amount1 = liquidity * balance1 / _total_supply;

        if amount0 == 0 || amount1 == 0 {
            return Err(UniswapV2Errors::InsufficientLiquidityBurned);
        }

        self._burn_from(this, liquidity)?;

        PSP22Ref::transfer(&this, _to, amount0, Vec::<u8>::new())?;
        PSP22Ref::transfer(&this, _to, amount1, Vec::<u8>::new())?;

        balance0 = PSP22Ref::balance_of(&token_0, this);
        balance1 = PSP22Ref::balance_of(&token_1, this);

        if fee_on.unwrap_or(false) {
            self.data::<PairStorage>().k_last = (self.data::<PairStorage>().reserve_0 as u128) * (self.data::<PairStorage>().reserve_1 as u128);
        }

        self.internal_update(balance0, balance1)?;

        Ok(())
    }



    fn swap(&mut self, _amount0: Balance, _amount1: Balance, _to: AccountId) -> Result<(), UniswapV2Errors> {
        let state = *self.data::<PairStorage>();
        let this = Self::env().account_id();

        if _amount0 == 0 && _amount1 == 0 {
            return Err(UniswapV2Errors::InsufficientAmountOut);
        }

        let (reserve_0, reserve_1, token_0, token_1) = (state.reserve_0, state.reserve_1, state.token_0, state.token_1);

        if _to == token_0 || _to == token_1 {
            return Err(UniswapV2Errors::InvalidToAddress);
        }

        PSP22Ref::transfer(&token_0, _to, _amount0, Vec::<u8>::new())?;
        PSP22Ref::transfer(&token_1, _to, _amount1, Vec::<u8>::new())?;

        let balance0 = PSP22Ref::balance_of(&token_0, this);
        let balance1 = PSP22Ref::balance_of(&token_1, this);

        let amount0_in = if balance0 > reserve_0 - _amount0 {
            balance0 - (reserve_0 - _amount0)
        } else {
            0u128
        };

        let amount1_in = if balance1 > reserve_1 - _amount1 {
            balance1 - (reserve_1 - _amount1)
        } else {
            0u128
        };

        if amount0_in == 0 && amount1_in == 0 {
            return Err(UniswapV2Errors::InsufficientAmountOut);
        }

        let balance0adjusted = balance0 * 1000 - amount0_in * 3;
        let balance1adjusted = balance1 * 1000 - amount1_in * 3;

        if (balance0adjusted * balance1adjusted) < (reserve_0 * reserve_1) * 1000u128.pow(2) {
            return Err(UniswapV2Errors::ConstantProductError);
        }

        self.internal_update(balance0, balance1)?;
        Ok(())
    }


    fn skim(&mut self, _to: AccountId) -> Result<(), UniswapV2Errors> {
        let state = *self.data::<PairStorage>();
        let this = Self::env().account_id();
        let (token_0, token_1) = (state.token_0, state.token_1);
        let balance0 = PSP22Ref::balance_of(&token_0, this);
        let balance1 = PSP22Ref::balance_of(&token_1, this);
        let amount0 = balance0 - state.reserve_0;
        let amount1 = balance1 - state.reserve_1;

        if amount0 > 0 {
            PSP22Ref::transfer(&token_0, _to, amount0, Vec::<u8>::new())?;
        }

        if amount1 > 0 {
            PSP22Ref::transfer(&token_1, _to, amount1, Vec::<u8>::new())?;
        }

        Ok(())
    }


    fn sync(&mut self) -> Result<(), UniswapV2Errors> {
        let state = *self.data::<PairStorage>();
        let (token_0, token_1) = (state.token_0, state.token_1);
        let balance0 = PSP22Ref::balance_of(&token_0, Self::env().account_id());
        let balance1 = PSP22Ref::balance_of(&token_1, Self::env().account_id());
        self.internal_update(balance0, balance1)?;
        Ok(())
    }





    // =========================================
    // Internal Functions 
    // =========================================
    fn internal_transfer(&mut self, _token: AccountId, _to: AccountId, _value: u128) -> bool {
        return PSP22Ref::transfer(&_token, _to, _value, Vec::<u8>::new()).is_ok();
    }

    fn internal_mint_fee(&mut self) -> Result<bool, UniswapV2Errors> {
        let state = self.data::<PairStorage>();
        let fee_to = Some(AccountId::from([1u8; 32])); // PSP22Ref::fee_to(&state.token_0); //TODO: fee_to

        let return_data = fee_to.is_some();

        if return_data {
            if state.k_last != 0 {
                let root_k_pre = (state.reserve_0 as u128) * (state.reserve_1 as u128);
                let root_k = internal_sqrt(root_k_pre);
                let root_k_last = internal_sqrt(state.k_last);

                if root_k > root_k_last {
                    let numerator = self._total_supply() * (root_k - root_k_last);
                    let denominator = (root_k * 5) + root_k_last;
                    let liquidity = numerator / denominator;
                    if liquidity > 0 {
                        self._mint_to(fee_to.unwrap(), liquidity)?;
                    }
                }
            }
        }
        Ok(return_data)
    }



    fn internal_update(&mut self, balance1: Balance, balance2: Balance) -> Result<(), UniswapV2Errors> {
        let state = self.data::<PairStorage>();
        let (reserve_0, reserve_1, block_timestamp_last) = (state.reserve_0, state.reserve_1, state.block_timestamp_last);
        let time_elapsed = Self::env().block_timestamp() as u128 - block_timestamp_last;

        if time_elapsed > 0 && reserve_0 != 0 && reserve_1 != 0 {
            state.price0_cumulative_last += (uq112x112decode(uq112x112encode(state.reserve_1), state.reserve_0)) * time_elapsed;
            state.price1_cumulative_last += (uq112x112decode(uq112x112encode(state.reserve_0), state.reserve_1)) * time_elapsed;
        }


        state.reserve_0 = balance1;
        state.reserve_1 = balance2;
        state.block_timestamp_last = Self::env().block_timestamp() as u128;


        // emit the sync event
        Ok(())
    }


}



fn internal_sqrt(_y: u128) -> u128 {
    let d1 = FixedU128::from_inner(_y);  
    let d2 = FixedU128::sqrt(d1); 
    let d3 = *d2.encode_as();
    d3 
}


fn uq112x112encode(x: u128) -> u128 {
    x * 2u128.pow(112)
}

fn uq112x112decode(x: u128, y: u128) -> u128 {
    x / y
}