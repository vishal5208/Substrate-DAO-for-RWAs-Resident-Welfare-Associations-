#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod rwa {
    use ink::storage::Mapping;

    #[derive(Clone, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub struct Proposal {
        creator: AccountId,
        for_votes: u128,
        against_votes: u128,
        executed: bool,
    }

    #[ink(storage)]
    pub struct Rwa {
        proposals: Mapping<u128, Proposal>,
        members: Mapping<AccountId, bool>,
        owner: AccountId,
        proposal_counter: u128,
    }

    impl Rwa {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                proposals: Default::default(),
                members: Default::default(),
                owner: Self::env().caller(),
                proposal_counter: 0,
            }
        }

        #[ink(message)]
        pub fn add_rwa_member(&mut self, account: AccountId) {
            if self.env().caller() == self.owner {
                self.members.insert(account, &true);
            }
        }

        #[ink(message)]
        pub fn remove_rwa_member(&mut self, account: AccountId) {
            if self.env().caller() == self.owner {
                self.members.insert(account, &false);
            }
        }

        #[ink(message)]
        pub fn create_proposal(&mut self) -> u128 {
            let caller = self.env().caller();
            let proposal_id: u128 = self.proposal_counter;

            // Increment proposal counter for the next proposal
            self.proposal_counter += 1;

            let new_proposal = Proposal {
                creator: caller,
                for_votes: 0,
                against_votes: 0,
                executed: false,
            };

            self.proposals.insert(proposal_id, &new_proposal);
            proposal_id
        }

        // vote for proposal
        #[ink(message)]
        pub fn vote(&mut self, proposal_id: u128, vote: bool) {
            let caller = self.env().caller();
            if let Some(mut proposal) = self.proposals.get(&proposal_id) {
                if let Some(is_member) = self.members.get(&caller) {
                    if !proposal.executed {
                        if is_member {
                            if vote {
                                proposal.for_votes += 1;
                            } else {
                                proposal.against_votes += 1;
                            }
                        }
                    }
                }
            }
        }

        #[ink(message)]
        pub fn execute_proposal(&mut self, proposal_id: u128) {
            let caller = self.env().caller();
            if caller == self.owner {
                if let Some(mut proposal) = self.proposals.get(&proposal_id) {
                    if !proposal.executed {
                        proposal.executed = true;
                    }
                }
            }
        }

        #[ink(message)]
        pub fn get_owner(&self) -> AccountId {
            self.owner
        }

        #[ink(message)]
        pub fn get_proposal_counter(&self) -> u128 {
            self.proposal_counter
        }

        #[ink(message)]
        pub fn get_proposal(&self, proposal_id: u128) -> Option<Proposal> {
            self.proposals.get(&proposal_id)
        }
    }
}
