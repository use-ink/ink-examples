#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod incrementer {
    // We import Mapping to store data for specific users (like a database)
    use ink::storage::Mapping;
    // We import H160, which is the 20-byte address format used in ink! v6 (PolkaVM)
    use ink::primitives::H160; 

    #[ink(storage)]
    pub struct Incrementer {
        // Global counter: This value is shared by everyone
        value: i32,
        // A map that links an Address (H160) to an i32 (Their personal number)
        my_value: Mapping<H160, i32>,
    }

    impl Incrementer {
        #[ink(constructor)]
        pub fn new(init_value: i32) -> Self {
            Self {
                value: init_value,
                my_value: Mapping::default(),
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(0)
        }

        /// Modifies the GLOBAL value.
        /// Everyone sees this change.
        #[ink(message)]
        pub fn inc(&mut self, by: i32) {
            self.value += by;
        }

        /// Reads the GLOBAL value.
        #[ink(message)]
        pub fn get(&self) -> i32 {
            self.value
        }

        /// Modifies the USER'S specific value.
        /// Only changes the number for the person calling this function.
        #[ink(message)]
        pub fn inc_mine(&mut self, by: i32) {
            // 1. Identify who called the contract (The Caller)
            let caller = self.env().caller();
            
            // 2. Get their current value from the map
            // If they haven't used it before, default to 0
            let my_val = self.my_value.get(caller).unwrap_or(0);
            
            // 3. Update the map with the new calculated value
            self.my_value.insert(caller, &(my_val + by));
        }

        /// Reads the USER'S specific value.
        /// Returns 0 if they have never interacted with the contract.
        #[ink(message)]
        pub fn get_mine(&self) -> i32 {
            // We need to know who is asking to return the correct number
            let caller = self.env().caller();
            self.my_value.get(caller).unwrap_or(0)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn default_works() {
            let contract = Incrementer::default();
            assert_eq!(contract.get(), 0);
        }

        #[ink::test]
        fn it_works() {
            let mut contract = Incrementer::new(42);
            // Global value checks
            assert_eq!(contract.get(), 42);
            contract.inc(5);
            assert_eq!(contract.get(), 47);
        }

        #[ink::test]
        fn my_value_works() {
            let mut contract = Incrementer::new(0);
            // In unit tests, the "caller" is usually a default account (Alice).
            // 1. Check initial state
            assert_eq!(contract.get_mine(), 0);
            
            // 2. Increment specific value
            contract.inc_mine(10);
            
            // 3. Check if it updated
            assert_eq!(contract.get_mine(), 10);
        }
    }
}
