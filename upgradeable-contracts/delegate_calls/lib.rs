#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod delegate_calls {
    use ink::{env::{
        call::{build_call, ExecutionInput, Selector},
        CallFlags, DefaultEnvironment,
    }, storage::traits::ManualKey};
    use ink::prelude::vec::Vec;
     use ink::storage::{Mapping, Lazy};

    /// Since we delegate a call, we need to match the storage layout
    /// of the deployed code we are delegating to.
    #[ink(storage)]
    pub struct DelegateCalls {
        /// Current leader of the DAO.
        leader: Lazy<AccountId, ManualKey<1>>,
        /// Members of the DAO.
        members: Mapping<AccountId, Balance, ManualKey<2>>,
        /// Number of members in the DAO
        member_count: Lazy<u128, ManualKey<3>>,
        /// Candidates for the re-election
        leader_candidates: Lazy<Vec<AccountId>, ManualKey<4>>,
        /// The hash of the on-chain code
        /// that is responsible for the re-election of the leader.
        election_code_hash: Lazy<Hash, ManualKey<5>>,
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
        pub fn new(election_code_hash: Hash) -> Result<Self, ExecutionError> {
            let leader = Self::env().caller();
            let mut members = Mapping::new();
            members.insert(leader, &1_000_000);
            let mut v = Self {
                leader: Lazy::new(),
                members,
                leader_candidates: Lazy::new(),
                member_count: Lazy::new(),
                election_code_hash: Lazy::new(),
            };
            v.leader.set(&leader);
            v.leader_candidates.set(&Vec::new());
            v.member_count.set(&1);
            v.election_code_hash.set(&election_code_hash);
            Ok(v)
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
            let leader = self.leader.get().unwrap();
            assert_ne!(leader, address);
            // ensure that the caller is the current leader
            assert_eq!(leader, caller);
            // update the member's balance
            self.members
                .insert(address, &new_balance)
                .ok_or(Error::ExecutionError)?;
            Ok(())
        }

        #[ink(message)]
        pub fn get_leader(&self) -> AccountId {
            self.leader.get().unwrap()
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
            let count = self.member_count.get_or_default() + 1;
            self.member_count.set(&count); 

            Ok(self.member_count.get_or_default())
        }

        #[ink(message)]
        pub fn nominate_for_election(&mut self) -> u128 {
            let caller = self.env().caller();
            // ensure the caller is the member
            assert!(self.members.contains(caller));
            // ensure that the account has not nominated itself already
            assert!(!self.leader_candidates.get().unwrap().contains(&caller));
            // ensure the current leader does not renominate itself
            let leader = self.leader.get().unwrap();
            assert_ne!(leader, caller);
            // update records
            let mut candidates = self.leader_candidates.get_or_default();
            candidates.push(caller);
            self.leader_candidates.set(&candidates);

            self.leader_candidates.get_or_default().len() as u128
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
            let hash = self.election_code_hash.get().ok_or(Error::ExecutionError)?;
            build_call::<DefaultEnvironment>()
                .delegate(hash)
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
            let hash = self.election_code_hash.get().ok_or(Error::ExecutionError)?;
            let result = build_call::<DefaultEnvironment>()
                .delegate(hash)
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
