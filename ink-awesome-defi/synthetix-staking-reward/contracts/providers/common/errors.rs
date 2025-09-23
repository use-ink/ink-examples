use openbrush::
    contracts::traits::{
        ownable::*,
        psp22::PSP22Error,
        errors::ReentrancyGuardError,
    };






#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum StakingRewardsErrors {
    OwnableError(OwnableError),
    ReentrancyGuardError(ReentrancyGuardError),
    NotAdmin,
    AmountShouldBeGreaterThanZero,
    InsufficientFunds,
    NotEnoughAllowance,
    TokenTransferFailed,
    Overflow,
    StakingStillInProgress,
    InvalidAmount,
    ProviderRewardIsToLarge,
    CannotWithdrawRewardToken,
    CannotWithdrawStakedToken,
}





impl From<OwnableError> for StakingRewardsErrors {
    fn from(error: OwnableError) -> Self {
        StakingRewardsErrors::OwnableError(error)
    }
}

impl From<ReentrancyGuardError> for StakingRewardsErrors {
    fn from(error: ReentrancyGuardError) -> Self {
        StakingRewardsErrors::ReentrancyGuardError(error)
    }
}