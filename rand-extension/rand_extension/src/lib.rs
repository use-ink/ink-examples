#![cfg_attr(not(feature = "std"), no_std, no_main)]

/// This is an example of how an ink! contract may call the Substrate
/// runtime function `RandomnessCollectiveFlip::random_seed`. See the
/// file `runtime/chain-extension-example.rs` for that implementation.
///
/// Here we define the operations to interact with the Substrate runtime.
#[ink::chain_extension]
pub trait FetchRandom {
    type ErrorCode = RandomReadErr;

    /// Note: this gives the operation a corresponding `func_id` (1101 in this case),
    /// and the chain-side chain extension will get the `func_id` to do further
    /// operations.
    #[ink(extension = 1101)]
    fn fetch_random(subject: [u8; 32]) -> [u8; 32];
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum RandomReadErr {
    FailGetRandomSource,
}

impl ink::env::chain_extension::FromStatusCode for RandomReadErr {
    fn from_status_code(status_code: u32) -> Result<(), Self> {
        match status_code {
            0 => Ok(()),
            1 => Err(Self::FailGetRandomSource),
            _ => panic!("encountered unknown status code"),
        }
    }
}
