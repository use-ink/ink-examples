#![cfg_attr(not(feature = "std"), no_std, no_main)]





#[openbrush::implementation(PSP22, PSP22Permit, Nonces, PSP22Mintable, Ownable, PSP22Metadata)]
#[openbrush::contract]
pub mod psp22_permit {
    use openbrush::{traits::Storage, modifiers};


    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct PSP22Lp {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        metadata: metadata::Data,
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        nonces: nonces::Data,
        #[storage_field]
        psp22_permit: psp22::extensions::permit::Data,
        
    }


    #[default_impl(PSP22Mintable)]
    #[modifiers(only_owner)]
    fn mint(&mut self) {}






    impl PSP22Lp {
        #[ink(constructor)]
        pub fn new(total_supply: Balance, name: Option<String>, symbol: Option<String>, decimal: u8) -> Self {
            let mut instance = Self::default();
            let caller = instance.env().caller();

            instance.metadata.name.set(&name);
            instance.metadata.symbol.set(&symbol);
            instance.metadata.decimals.set(&decimal);

            ownable::Internal::_init_with_owner(&mut instance, Self::env().caller());
            psp22::Internal::_mint_to(&mut instance, caller, total_supply).expect("Should mint total_supply");
            

            instance
        }



        #[ink(message)]
        pub fn burn(&mut self, _amount: Balance) -> Result<(), PSP22Error> {
            let caller = self.env().caller();
            psp22::Internal::_burn_from(self, caller, _amount)
        }
    }
}
