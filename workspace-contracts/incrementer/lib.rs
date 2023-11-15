#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod incrementer {
    #[ink(storage)]
    pub struct Incrementer {
        value: i32,
    }

    impl Incrementer {
        #[ink(constructor)]
        pub fn new(init_value: i32) -> Self {
            Self { value: init_value }
        }

        #[ink(constructor)]
        pub fn new_default() -> Self {
            Self::new(Default::default())
        }

        #[ink(message)]
        pub fn inc(&mut self, by: i32) {
            self.value = self.value.saturating_add(by);
        }

        #[ink(message)]
        pub fn get(&self) -> i32 {
            self.value
        }
    }
}
