#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod delegate_calls {
    use ink::{env::{
        call::{build_call, ExecutionInput, Selector},
        CallFlags, DefaultEnvironment,
    }, storage::traits::ManualKey};
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;

    /// Since we delegate a call, we need to match the storage layout
    /// of the deployed code we are delegating to.
    #[ink(storage)]
    pub struct DelegateCalls {
        /// Current leader of the DAO.
        leader: AccountId,
        /// Members of the DAO.
        members: Mapping<AccountId, Balance, ManualKey<200>>,
        /// Number of members in the DAO
        member_count: u128,
        /// Candidates for the re-election
        leader_candidates: Vec<AccountId>,
        /// The hash of the on-chain code
        /// that is responsible for the re-election of the leader.
        election_code_hash: Hash,
    }

    #[derive(scale::Encode, scale::Decode, Debug)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        ExecutionError,
        DelegateError,
    }

    #[derive(scale::Encode, scale::Decode, Debug)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum ExecutionError {
        Election,
        CodeHashUpdate,
        Generic,
    }

    impl DelegateCalls {
        #[ink(constructor)]
        pub fn new(election_code_hash: Hash, leader: AccountId) -> Result<Self, Error> {
            let mut members = Mapping::new();
            members.insert(leader, &1_000_000);
            Ok(Self {
                leader,
                members,
                leader_candidates: Vec::new(),
                member_count: 1u128,
                election_code_hash,
            })
        }

        #[ink(message)]
        pub fn set_member_balance(
            &mut self,
            address: AccountId,
            new_balance: Balance,
        ) -> Result<(), Error> {
            let caller = self.env().caller();
            // ensure that the given account is the member of the DAO
            assert!(self.members.get(address).is_some());
            // ensure that the caller is not the leader so it can't change their own balance
            assert_ne!(self.leader, address);
            // ensure that the caller is the current leader
            assert_eq!(caller, self.leader);
            // update the member's balance
            self.members
                .insert(address, &new_balance)
                .ok_or(Error::ExecutionError)?;
            Ok(())
        }

        #[ink(message)]
        pub fn get_leader(&self) -> AccountId {
            self.leader
        }

        #[ink(message)]
        pub fn get_member_balance(&self, address: AccountId) -> Option<Balance> {
            self.members.get(address)
        }

        #[ink(message)]
        pub fn join(&mut self) -> Result<u128, Error> {
            let caller = self.env().caller();
            //ensure that the account is not the member already
            assert!(!self.members.contains(caller));
            //update records
            self.members.insert(caller, &1_000_000);
            self.member_count += 1;

            Ok(self.member_count)
        }

        #[ink(message)]
        pub fn nominate_for_election(&mut self) -> u128 {
            let caller = self.env().caller();
            // ensure the caller is the member
            assert!(self.members.contains(caller));
            // ensure that the account has not nominated itself already
            assert!(!self.leader_candidates.contains(&caller));
            // ensure the current leader does not renominate itself
            assert_ne!(self.leader, caller);
            // update records
            self.leader_candidates.push(caller);

            self.leader_candidates.len() as u128
        }

        /// Update the hash of the code
        ///
        /// We also delegate this logic,
        /// in case we need to update the decision making
        /// behind this operation.
        #[ink(message)]
        pub fn update_election_codehash(
            &mut self,
            _code_hash: Hash,
        ) -> Result<(), Error> {
            let selector = ink::selector_bytes!("update_logic");
            build_call::<DefaultEnvironment>()
                .delegate(self.election_code_hash)
                .call_flags(
                    CallFlags::default()
                        // forward code_hash directly to the callee
                        .set_forward_input(true)
                        // restrict reentrance, so that the caller does not maliciously calls this message to update the code
                        .set_allow_reentry(false),
                )
                .exec_input(ExecutionInput::new(Selector::new(selector)))
                .returns::<AccountId>()
                .try_invoke()
                .map_err(|_| Error::DelegateError)?
                .map_err(|_| Error::ExecutionError)?;
            Ok(())
        }

        /// Initiates the re-election of the leader.
        ///
        /// This message can be invoked by any member of the DAO.
        /// The logic is delegated to the different deployed code on-chain.
        #[ink(message)]
        pub fn initiated_delegated_election(&mut self) -> Result<AccountId, Error> {
            let selector = ink::selector_bytes!("elect");
            let result = build_call::<DefaultEnvironment>()
                .delegate(self.election_code_hash)
                .exec_input(ExecutionInput::new(Selector::new(selector)))
                .returns::<Result<AccountId, ExecutionError>>()
                .try_invoke()
                .map_err(|_| Error::DelegateError)?
                .map_err(|_| Error::ExecutionError)?
                .map_err(|_| Error::ExecutionError)?;
            Ok(result)
        }
    }

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
    }
}
