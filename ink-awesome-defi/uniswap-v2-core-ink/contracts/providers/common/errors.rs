use openbrush::
    contracts::traits::{
        ownable::*,
        psp22::PSP22Error,
        errors::ReentrancyGuardError,
    };






#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum UniswapV2Errors {
    PSP22Error(PSP22Error),
    OwnableError(OwnableError),
    ReentrancyGuardError(ReentrancyGuardError),
    InsufficientLiquidityMinted,
    InsufficientLiquidityBurned,
    InsufficientAmountOut,
    InvalidToAddress,
    ConstantProductError,
    InvalidPairAddress,
    PairAlreadyExists
}




// implementing error conversion utils
impl From<PSP22Error> for UniswapV2Errors {
    fn from(error: PSP22Error) -> Self {
        UniswapV2Errors::PSP22Error(error)
    }
}

impl From<OwnableError> for UniswapV2Errors {
    fn from(error: OwnableError) -> Self {
        UniswapV2Errors::OwnableError(error)
    }
}

impl From<ReentrancyGuardError> for UniswapV2Errors {
    fn from(error: ReentrancyGuardError) -> Self {
        UniswapV2Errors::ReentrancyGuardError(error)
    }
}