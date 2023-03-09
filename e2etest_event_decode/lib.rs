#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod testing_event_decode {

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct TestingEventDecode {
        /// Stores a single `bool` value on the storage.
        value: bool,
    }

    /// Emitted whenever an address changes.
    #[ink(event)]
    pub struct FlipperEvent {
        #[ink(topic)]
        caller: AccountId,
        #[ink(topic)]
        value: bool,
    }

    impl TestingEventDecode {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn flip(&mut self) {
            let caller = self.env().caller();
            self.value = !self.value;

            // Add `FlipperEvent`
            self.env().emit_event(FlipperEvent {
                caller,
                value: self.value,
            });
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        use ink::env::test::EmittedEvent;

        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        type Event = <TestingEventDecode as ::ink::reflect::ContractEventBase>::Type;

        fn decode_events(emitted_events: Vec<EmittedEvent>) -> Vec<Event> {
            emitted_events
                .into_iter()
                .map(|event| {
                    <Event as scale::Decode>::decode(&mut &event.data[..])
                        .expect("Invalid data")
                })
                .collect()
        }

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let testing_event_decode = TestingEventDecode::default();
            assert_eq!(testing_event_decode.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            // given
            let accounts =
                ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);

            let mut testing_event_decode = TestingEventDecode::new(false);
            assert_eq!(testing_event_decode.get(), false);
            testing_event_decode.flip();

            // decode event
            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            let decode_events = decode_events(emitted_events);

            match &decode_events[0] {
                Event::FlipperEvent(FlipperEvent { caller, value }) => {
                    // check event emitted by the `flip` function
                    assert_eq!(*value, true);
                    assert_eq!(*caller, accounts.alice);
                }
                _ => (),
            }

            assert_eq!(testing_event_decode.get(), true);
        }
    }

    /// This is how you'd write end-to-end (E2E) or integration tests for ink! contracts.
    ///
    /// When running these you need to make sure that you:
    /// - Compile the tests with the `e2e-tests` feature flag enabled (`--features e2e-tests`)
    /// - Are running a Substrate node which contains `pallet-contracts` in the background
    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// A helper function used for calling contract messages.
        use ink_e2e::build_message;

        /// The End-to-End test `Result` type.
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        /// We test that we can upload and instantiate the contract using its default constructor.
        #[ink_e2e::test]
        async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = TestingEventDecodeRef::default();

            // When
            let contract_account_id = client
                .instantiate(
                    "testing_event_decode",
                    &ink_e2e::alice(),
                    constructor,
                    0,
                    None,
                )
                .await
                .expect("instantiate failed")
                .account_id;

            // Then
            let get = build_message::<TestingEventDecodeRef>(contract_account_id.clone())
                .call(|testing_event_decode| testing_event_decode.get());
            let get_result = client.call_dry_run(&ink_e2e::alice(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            Ok(())
        }

        /// We test that we can read and write a value from the on-chain contract contract.
        #[ink_e2e::test]
        async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let bob = ink_e2e::account_id(ink_e2e::AccountKeyring::Bob);

            let constructor = TestingEventDecodeRef::new(false);
            let contract_account_id = client
                .instantiate(
                    "testing_event_decode",
                    &ink_e2e::bob(),
                    constructor,
                    0,
                    None,
                )
                .await
                .expect("instantiate failed")
                .account_id;

            let get = build_message::<TestingEventDecodeRef>(contract_account_id.clone())
                .call(|testing_event_decode| testing_event_decode.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            // When
            let flip =
                build_message::<TestingEventDecodeRef>(contract_account_id.clone())
                    .call(|testing_event_decode| testing_event_decode.flip());
            let _flip_result = client
                .call(&ink_e2e::bob(), flip, 0, None)
                .await
                .expect("flip failed");

            // Filter the events
            let contract_emitted_event = _flip_result
                .events
                .iter()
                .find(|event| {
                    event
                        .as_ref()
                        .expect("Expect Event")
                        .event_metadata()
                        .event()
                        == "ContractEmitted"
                })
                .expect("Expect ContractEmitted event")
                .unwrap();

            // Decode the expected event type
            let event = contract_emitted_event.field_bytes();
            let decode_event = <FlipperEvent as scale::Decode>::decode(&mut &event[34..])
                .expect("invalid data");

            // Destructor
            let FlipperEvent { caller, value } = decode_event;

            // check event emitted by the `flip` function
            assert_eq!(caller, bob);
            assert_eq!(value, true);

            // Then
            let get = build_message::<TestingEventDecodeRef>(contract_account_id.clone())
                .call(|testing_event_decode| testing_event_decode.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), true));

            Ok(())
        }
    }
}
