#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod fair_election {
    use ink::prelude::vec::Vec;
    use ink::storage::traits::ManualKey;
    use ink::storage::{Lazy, Mapping};

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct FairElection {
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
    pub enum ExecutionError {
        Election,
        CodeHashUpdate,
        Generic,
    }

    impl FairElection {
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
        pub fn elect(&mut self) -> Result<AccountId, ExecutionError> {
            // we simply elect an account with highest balance
            let candidates = self
                .leader_candidates
                .get()
                .ok_or(ExecutionError::Generic)?;
            let leader = candidates
                .iter()
                .max_by(|x, y| {
                    let x_balance = self.members.get(x).unwrap_or_default();
                    let y_balance = self.members.get(y).unwrap_or_default();
                    x_balance.cmp(&y_balance)
                })
                .ok_or(ExecutionError::Election)?;

            //update records
            self.leader.set(leader);
            self.leader_candidates.set(&Vec::new());

            self.leader.get().ok_or(ExecutionError::Generic)
        }

        #[ink(message)]
        pub fn update_logic(
            &mut self,
            election_code_hash: Hash,
        ) -> Result<(), ExecutionError> {
            // ensure we code hash is not the same
            let local_hash = self.election_code_hash.get_or_default();
            assert_ne!(local_hash, election_code_hash);
            // for simplicity, we just check that the election is not in progress
            // and then update thr code
            assert!(self.leader_candidates.get_or_default().is_empty());
            self.election_code_hash.set(&election_code_hash);

            Ok(())
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
    }
}
