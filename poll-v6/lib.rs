#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod poll {
    use ink::storage::Mapping;
    use ink::primitives::H160;

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        NotAdmin,
        PollNotActive,
        AlreadyVoted,
        PollAlreadyActive,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    #[ink(event)]
    pub struct Voted {
        #[ink(topic)]
        voter: H160,
        choice: bool, // true = yes, false = no
    }

    #[ink(event)]
    pub struct PollStateChanged {
        is_active: bool,
    }

    #[ink(storage)]
    pub struct Poll {
        /// The boss. Only this account can open/close polls.
        admin: H160,
        /// Is voting currently allowed?
        is_active: bool,
        /// Total "Yes" votes.
        yes_votes: u32,
        /// Total "No" votes.
        no_votes: u32,
        /// Tracks who has voted to prevent double voting.
        has_voted: Mapping<H160, bool>,
    }

    impl Poll {
        #[ink(constructor)]
        pub fn new() -> Self {
            let caller = Self::env().caller();
            Self {
                admin: caller,
                is_active: false, // Poll starts closed
                yes_votes: 0,
                no_votes: 0,
                has_voted: Mapping::default(),
            }
        }

        /// ADMIN ONLY: Starts a new poll (resets counters).
        #[ink(message)]
        pub fn start_poll(&mut self) -> Result<()> {
            self.ensure_admin()?;
            
            if self.is_active {
                return Err(Error::PollAlreadyActive);
            }

            // Reset state for new poll
            self.is_active = true;
            self.yes_votes = 0;
            self.no_votes = 0;
            // Note: In a real app, clearing a Mapping is expensive. 
            // Usually, we would use a PollID to separate polls. 
            // For this simple example, we accept previous voters can't vote again 
            // or we manually assume a reset logic (omitted for simplicity).
            
            self.env().emit_event(PollStateChanged { is_active: true });
            Ok(())
        }

        /// ADMIN ONLY: Stops the poll.
        #[ink(message)]
        pub fn end_poll(&mut self) -> Result<()> {
            self.ensure_admin()?;
            self.is_active = false;
            self.env().emit_event(PollStateChanged { is_active: false });
            Ok(())
        }

        /// PUBLIC: Vote Yes (true) or No (false).
        #[ink(message)]
        pub fn vote(&mut self, choice: bool) -> Result<()> {
            if !self.is_active {
                return Err(Error::PollNotActive);
            }

            let caller = self.env().caller();
            if self.has_voted.contains(caller) {
                return Err(Error::AlreadyVoted);
            }

            // Record vote
            if choice {
                self.yes_votes += 1;
            } else {
                self.no_votes += 1;
            }

            // Mark voter as "done"
            self.has_voted.insert(caller, &true);

            self.env().emit_event(Voted {
                voter: caller,
                choice,
            });

            Ok(())
        }

        /// PUBLIC: View current results.
        #[ink(message)]
        pub fn get_results(&self) -> (u32, u32) {
            (self.yes_votes, self.no_votes)
        }

        /// PUBLIC: Check if poll is active.
        #[ink(message)]
        pub fn is_active(&self) -> bool {
            self.is_active
        }

        // --- Helper to protect Admin functions ---
        fn ensure_admin(&self) -> Result<()> {
            if self.env().caller() != self.admin {
                Err(Error::NotAdmin)
            } else {
                Ok(())
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn admin_can_start_poll() {
            let mut poll = Poll::new();
            assert_eq!(poll.is_active(), false);
            
            assert_eq!(poll.start_poll(), Ok(()));
            assert_eq!(poll.is_active(), true);
        }

        #[ink::test]
        fn voting_works() {
            let mut poll = Poll::new();
            poll.start_poll().unwrap();

            // Vote Yes
            assert_eq!(poll.vote(true), Ok(()));
            
            // Check results: (1 Yes, 0 No)
            assert_eq!(poll.get_results(), (1, 0));
        }

        #[ink::test]
        fn double_voting_fails() {
            let mut poll = Poll::new();
            poll.start_poll().unwrap();

            assert_eq!(poll.vote(true), Ok(()));
            assert_eq!(poll.vote(false), Err(Error::AlreadyVoted));
        }
    }
}
