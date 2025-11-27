#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod anonymous_poll {
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;
    use ink::env::call::{build_call_sol, ExecutionInput, Selector};
    use ink::env::DefaultEnvironment;
    use ink::H160;

    /// Storage for the anonymous poll contract
    #[ink(storage)]
    pub struct AnonymousPoll {
        /// Address of the deployed Solidity verifier contract
        verifier_address: H160,
        /// Contract owner/admin
        owner: H160,
        /// Counter for poll IDs
        next_poll_id: u128,
        /// Mapping from poll_id to Poll data
        polls: Mapping<u128, Poll>,
        /// Mapping from (poll_id, nullifier_hash) to bool - tracks used nullifiers
        used_nullifiers: Mapping<(u128, [u8; 32]), bool>,
        /// Mapping from poll_id to vote tallies (option_index -> vote_count)
        vote_tallies: Mapping<(u128, u32), u32>,
    }

    /// Poll structure
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(
        feature = "std",
        derive(ink::storage::traits::StorageLayout)
    )]
    pub struct Poll {
        /// Unique poll identifier
        pub id: u128,
        /// Poll title/question
        pub title: String,
        /// Poll description
        pub description: String,
        /// Merkle root of eligible voters
        pub merkle_root: [u8; 32],
        /// Number of voting options
        pub max_options: u32,
        /// Poll creator
        pub creator: H160,
        /// Whether the poll is active
        pub is_active: bool,
        /// Total votes cast
        pub total_votes: u32,
        /// Poll creation timestamp
        pub created_at: u64,
        /// Poll end timestamp (0 means no end time)
        pub ends_at: u64,
    }

    /// Errors
    #[ink::error]
    pub enum Error {
        /// Only the owner can perform this action
        OnlyOwner,
        /// Poll does not exist
        PollNotFound,
        /// Poll is not active
        PollNotActive,
        /// Poll has ended
        PollEnded,
        /// Nullifier has already been used (double voting attempt)
        NullifierAlreadyUsed,
        /// Proof verification failed
        ProofVerificationFailed,
        /// Cross-contract call to verifier failed
        VerifierCallFailed,
        /// Invalid vote option
        InvalidVoteOption,
        /// Invalid merkle root
        InvalidMerkleRoot,
        /// Invalid poll parameters
        InvalidPollParameters,
    }

    /// Events
    #[ink(event)]
    pub struct PollCreated {
        #[ink(topic)]
        poll_id: u128,
        creator: H160,
        max_options: u32,
    }

    #[ink(event)]
    pub struct VoteCast {
        #[ink(topic)]
        poll_id: u128,
        nullifier: [u8; 32],
        timestamp: u64,
    }

    #[ink(event)]
    pub struct PollClosed {
        #[ink(topic)]
        poll_id: u128,
        total_votes: u32,
    }

    impl AnonymousPoll {
        /// Constructor
        #[ink(constructor)]
        pub fn new(verifier_address: H160) -> Self {
            let caller = Self::env().caller();
            Self {
                verifier_address,
                owner: caller,
                next_poll_id: 1,
                polls: Mapping::default(),
                used_nullifiers: Mapping::default(),
                vote_tallies: Mapping::default(),
            }
        }

        /// Create a new poll
        #[ink(message)]
        pub fn create_poll(
            &mut self,
            title: String,
            description: String,
            merkle_root: [u8; 32],
            max_options: u32,
            duration_seconds: u64,
        ) -> Result<u128, Error> {
            // Enforce string length limits to prevent payload size issues
            if title.len() > 100 {
                return Err(Error::InvalidPollParameters);
            }
            
            if description.len() > 500 {
                return Err(Error::InvalidPollParameters);
            }
            
            if max_options == 0 || max_options > 100 {
                return Err(Error::InvalidPollParameters);
            }

            if merkle_root == [0u8; 32] {
                return Err(Error::InvalidMerkleRoot);
            }

            let poll_id = self.next_poll_id;
            let caller = self.env().caller();
            let now = self.env().block_timestamp();
            let ends_at = if duration_seconds > 0 {
                now + duration_seconds * 1000 // Convert to milliseconds
            } else {
                0 // No end time
            };

            let poll = Poll {
                id: poll_id,
                title: title.clone(),
                description,
                merkle_root,
                max_options,
                creator: caller,
                is_active: true,
                total_votes: 0,
                created_at: now,
                ends_at,
            };

            self.polls.insert(poll_id, &poll);
            self.next_poll_id += 1;

            // Initialize vote tallies for all options
            for option in 0..max_options {
                self.vote_tallies.insert((poll_id, option), &0);
            }

            self.env().emit_event(PollCreated {
                poll_id,
                creator: caller,
                max_options,
            });

            Ok(poll_id)
        }

        /// Cast a vote with zero-knowledge proof
        #[ink(message)]
        pub fn cast_vote(
            &mut self,
            poll_id: u128,
            proof: Vec<u8>,
            nullifier: [u8; 32],
            vote_choice: u32,
        ) -> Result<(), Error> {
            // Get poll and validate
            let mut poll = self.polls.get(poll_id).ok_or(Error::PollNotFound)?;
            
            if !poll.is_active {
                return Err(Error::PollNotActive);
            }

            // Check if poll has ended
            let now = self.env().block_timestamp();
            if poll.ends_at > 0 && now > poll.ends_at {
                return Err(Error::PollEnded);
            }

            // Check if vote option is valid
            if vote_choice >= poll.max_options {
                return Err(Error::InvalidVoteOption);
            }

            // Check if nullifier has been used (prevents double voting)
            if self.used_nullifiers.get((poll_id, nullifier)).unwrap_or(false) {
                return Err(Error::NullifierAlreadyUsed);
            }

            // Prepare public inputs for verification
            // Public inputs: [merkle_root, nullifier, poll_id, max_options]
            let public_inputs = self.encode_public_inputs(
                poll.merkle_root,
                nullifier,
                poll_id,
                poll.max_options,
            );

            // Call Solidity verifier contract
            let verification_result = self.call_verifier(proof, public_inputs)?;

            if !verification_result {
                return Err(Error::ProofVerificationFailed);
            }

            // Mark nullifier as used
            self.used_nullifiers.insert((poll_id, nullifier), &true);

            // Update vote tally
            let current_tally = self.vote_tallies.get((poll_id, vote_choice)).unwrap_or(0);
            self.vote_tallies.insert((poll_id, vote_choice), &(current_tally + 1));

            // Update poll total votes
            poll.total_votes += 1;
            self.polls.insert(poll_id, &poll);

            // Emit event
            self.env().emit_event(VoteCast {
                poll_id,
                nullifier,
                timestamp: now,
            });

            Ok(())
        }

        /// Call the Solidity verifier contract
        fn call_verifier(
            &self,
            proof: Vec<u8>,
            public_inputs: Vec<u8>,
        ) -> Result<bool, Error> {

            /// Note that this is a const function, it is evaluated at compile time.
            const fn solidity_selector(fn_sig: &str) -> [u8; 4] {
                let output: [u8; 32] = const_crypto::sha3::Keccak256::new()
                    .update(fn_sig.as_bytes())
                    .finalize();
                [output[0], output[1], output[2], output[3]]
            }
            // Solidity function signature: verify(bytes calldata proof, bytes calldata publicInputs)
            let selector = solidity_selector("verify(bytes,bytes)");

            let result = build_call_sol::<DefaultEnvironment>()
                .call(self.verifier_address)
                .exec_input(
                    ExecutionInput::new(Selector::new(selector))
                        .push_arg(proof)
                        .push_arg(public_inputs)
                )
                .returns::<bool>()
                .try_invoke();

            match result {
                Ok(Ok(is_valid)) => Ok(is_valid),
                Ok(Err(_)) => Err(Error::VerifierCallFailed),
                Err(_) => Err(Error::VerifierCallFailed),
            }
        }

        /// Encode public inputs for the verifier
        /// Format: concatenate all public inputs as bytes
        fn encode_public_inputs(
            &self,
            merkle_root: [u8; 32],
            nullifier: [u8; 32],
            poll_id: u128,
            max_options: u32,
        ) -> Vec<u8> {
            let mut inputs = Vec::new();
            
            // Add merkle_root (32 bytes)
            inputs.extend_from_slice(&merkle_root);
            
            // Add nullifier (32 bytes)
            inputs.extend_from_slice(&nullifier);
            
            // Add poll_id (16 bytes for u128)
            inputs.extend_from_slice(&poll_id.to_be_bytes());
            
            // Add max_options (4 bytes for u32)
            inputs.extend_from_slice(&max_options.to_be_bytes());
            
            inputs
        }

        /// Close a poll (only creator or owner)
        #[ink(message)]
        pub fn close_poll(&mut self, poll_id: u128) -> Result<(), Error> {
            let caller = self.env().caller();
            let mut poll = self.polls.get(poll_id).ok_or(Error::PollNotFound)?;

            // Only poll creator or contract owner can close
            if caller != poll.creator && caller != self.owner {
                return Err(Error::OnlyOwner);
            }

            poll.is_active = false;
            self.polls.insert(poll_id, &poll);

            self.env().emit_event(PollClosed {
                poll_id,
                total_votes: poll.total_votes,
            });

            Ok(())
        }

        /// Get poll details
        #[ink(message)]
        pub fn get_poll(&self, poll_id: u128) -> (bool, u128, String, String, [u8; 32], u32, H160, bool, u32, u64, u64) {
            match self.polls.get(poll_id) {
                Some(poll) => (
                    true,  // exists
                    poll.id,
                    poll.title,
                    poll.description,
                    poll.merkle_root,
                    poll.max_options,
                    poll.creator,
                    poll.is_active,
                    poll.total_votes,
                    poll.created_at,
                    poll.ends_at,
                ),
                None => (
                    false,  // doesn't exist
                    0,
                    String::new(),
                    String::new(),
                    [0u8; 32],
                    0,
                    H160::from([0u8; 20]),
                    false,
                    0,
                    0,
                    0,
                ),
            }
        }

        /// Get vote tally for a specific option
        #[ink(message)]
        pub fn get_vote_tally(&self, poll_id: u128, option: u32) -> u32 {
            self.vote_tallies.get((poll_id, option)).unwrap_or(0)
        }

        /// Get all vote tallies for a poll
        #[ink(message)]
        pub fn get_all_tallies(&self, poll_id: u128) -> Result<Vec<u32>, Error> {
            let poll = self.polls.get(poll_id).ok_or(Error::PollNotFound)?;
            let mut tallies = Vec::new();

            for option in 0..poll.max_options {
                let tally = self.vote_tallies.get((poll_id, option)).unwrap_or(0);
                tallies.push(tally);
            }

            Ok(tallies)
        }

        /// Check if a nullifier has been used
        #[ink(message)]
        pub fn is_nullifier_used(&self, poll_id: u128, nullifier: [u8; 32]) -> bool {
            self.used_nullifiers.get((poll_id, nullifier)).unwrap_or(false)
        }

        /// Get the verifier contract address
        #[ink(message)]
        pub fn get_verifier_address(&self) -> H160 {
            self.verifier_address
        }

        /// Update verifier address (only owner)
        #[ink(message)]
        pub fn update_verifier(&mut self, new_verifier: H160) -> Result<(), Error> {
            if self.env().caller() != self.owner {
                return Err(Error::OnlyOwner);
            }
            self.verifier_address = new_verifier;
            Ok(())
        }

        /// Get contract owner
        #[ink(message)]
        pub fn get_owner(&self) -> H160 {
            self.owner
        }

        /// Get total number of polls
        #[ink(message)]
        pub fn get_total_polls(&self) -> u128 {
            self.next_poll_id - 1
        }

        /// Check if poll has ended
        #[ink(message)]
        pub fn has_poll_ended(&self, poll_id: u128) -> Result<bool, Error> {
            let poll = self.polls.get(poll_id).ok_or(Error::PollNotFound)?;
            let now = self.env().block_timestamp();
            Ok(poll.ends_at > 0 && now > poll.ends_at)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn test_new_contract() {
            let accounts = ink::env::test::default_accounts();
            let contract = AnonymousPoll::new(accounts.bob);
            
            assert_eq!(contract.get_verifier_address(), accounts.bob);
            assert_eq!(contract.get_owner(), accounts.alice);
            assert_eq!(contract.get_total_polls(), 0);
        }

        #[ink::test]
        fn test_create_poll() {
            let accounts = ink::env::test::default_accounts();
            let mut contract = AnonymousPoll::new(accounts.bob);

            let merkle_root = [1u8; 32];
            let result = contract.create_poll(
                String::from("Test Poll"),
                String::from("This is a test poll"),
                merkle_root,
                3,
                86400, // 1 day
            );

            assert!(result.is_ok());
            let poll_id = result.unwrap();
            assert_eq!(poll_id, 1);

            let (exists, id, title, _, _, max_options, _, is_active, _, _, _) = contract.get_poll(poll_id);
            assert!(exists);
            assert_eq!(id, 1);
            assert_eq!(title, "Test Poll");
            assert_eq!(max_options, 3);
            assert_eq!(is_active, true);
        }

        #[ink::test]
        fn test_invalid_poll_parameters() {
            let accounts = ink::env::test::default_accounts();
            let mut contract = AnonymousPoll::new(accounts.bob);

            // Test with 0 options
            let result = contract.create_poll(
                String::from("Invalid Poll"),
                String::from("This poll has no options"),
                [1u8; 32],
                0,
                0,
            );
            assert!(result.is_err());

            // Test with invalid merkle root
            let result = contract.create_poll(
                String::from("Invalid Poll"),
                String::from("This poll has invalid merkle root"),
                [0u8; 32],
                3,
                0,
            );
            assert!(result.is_err());
        }

        #[ink::test]
        fn test_get_vote_tallies() {
            let accounts = ink::env::test::default_accounts();
            let mut contract = AnonymousPoll::new(accounts.bob);

            let poll_id = contract.create_poll(
                String::from("Test Poll"),
                String::from("Test"),
                [1u8; 32],
                3,
                0,
            ).unwrap();

            // All tallies should start at 0
            let tallies = contract.get_all_tallies(poll_id).unwrap();
            assert_eq!(tallies.len(), 3);
            assert_eq!(tallies[0], 0);
            assert_eq!(tallies[1], 0);
            assert_eq!(tallies[2], 0);
        }

        #[ink::test]
        fn test_nullifier_tracking() {
            let accounts = ink::env::test::default_accounts();
            let contract = AnonymousPoll::new(accounts.bob);

            let nullifier = [42u8; 32];
            let poll_id = 1;

            // Nullifier should not be used initially
            assert_eq!(contract.is_nullifier_used(poll_id, nullifier), false);
        }
    }
}