#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod wrapper {
    use flipper::flipper::FlipperRef;
    use ink::env::call::FromAccountId;
    use ink::ToAccountId;

    #[ink(storage)]
    pub struct Wrapper {
        flipper_account: AccountId,
    }

    impl Wrapper {
        #[ink(constructor)]
        pub fn new(flipper_code_hash: Hash) -> Self {
            let flipper = FlipperRef::new(true)
                .code_hash(flipper_code_hash)
                .endowment(0)
                .salt_bytes([0xDE, 0xAD, 0xBE, 0xEF])
                .instantiate();

            Self { 
                flipper_account: flipper.to_account_id() 
            }
        }

        #[ink(message)]
        pub fn proxy_flip(&mut self) {
            let mut flipper: FlipperRef = FromAccountId::from_account_id(self.flipper_account);
            flipper.flip();
        }

        #[ink(message)]
        pub fn proxy_get(&self) -> bool {
            let flipper: FlipperRef = FromAccountId::from_account_id(self.flipper_account);
            flipper.get()
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        use ink_e2e::ContractsBackend;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn cross_contract_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // 1. Upload Flipper Code
            let flipper_hash = client
                .upload("flipper", &ink_e2e::alice())
                .submit()
                .await
                .expect("uploading flipper failed")
                .code_hash;

            // 2. Instantiate Wrapper
            let mut constructor = WrapperRef::new(flipper_hash);
            
            // We keep the whole contract object, not just the account_id
            let wrapper_contract = client
                .instantiate("wrapper", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("wrapper instantiate failed");

            // 3. Verify Initial State
            // We use .call_builder::<Wrapper>() to get a builder that returns CallBuilder objects
            let get_call = wrapper_contract.call_builder::<Wrapper>().proxy_get();
            
            let get_res = client
                .call(&ink_e2e::alice(), &get_call)
                .submit()
                .await
                .expect("proxy_get failed");
            
            assert_eq!(get_res.return_value(), true);

            // 4. Execute Flip
            let flip_call = wrapper_contract.call_builder::<Wrapper>().proxy_flip();

            let _ = client
                .call(&ink_e2e::alice(), &flip_call)
                .submit()
                .await
                .expect("proxy_flip failed");

            // 5. Verify State Changed
            let get_call_2 = wrapper_contract.call_builder::<Wrapper>().proxy_get();
                
            let get_res_2 = client
                .call(&ink_e2e::alice(), &get_call_2)
                .submit()
                .await
                .expect("proxy_get failed");
                
            assert_eq!(get_res_2.return_value(), false);

            Ok(())
        }
    }
}