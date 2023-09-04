#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod rwa {
    use ink::storage::Mapping;
    use scale::alloc::vec::Vec;

    #[derive(Clone, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub struct Proposal {
        creator: AccountId,
        for_votes: u128,
        executed: bool,
    }

    #[derive(Clone, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    #[derive(Debug)]
    pub struct Dao {
        voting_power: Vec<(AccountId, u128)>,
        quorum: u128,
        voting_period: u128,
    }

    #[ink(storage)]
    pub struct Rwa {
        proposals: Mapping<u128, Proposal>,
        owner: AccountId,
        proposal_counter: u128,
        dao: Dao,
    }

    impl Rwa {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                proposals: Default::default(),
                owner: Self::env().caller(),
                proposal_counter: 0,
                dao: Dao {
                    voting_power: Default::default(),
                    quorum: 10,
                    voting_period: 300,
                },
            }
        }

        #[ink(message)]
        pub fn add_rwa_dao_member(&mut self, account: AccountId, power: u128) {
            if self.env().caller() == self.owner {
                let voting_power_entry = (account, power);
                self.dao.voting_power.push(voting_power_entry);
            }
        }

        #[ink(message)]
        pub fn remove_rwa_dao_member(&mut self, account: AccountId) {
            let caller = self.env().caller();
            if caller == self.owner {
                // Find and remove the member from the `dao.voting_power` vector
                let voting_power_index = self.dao.voting_power
                    .iter()
                    .position(|(acc, _)| *acc == account);

                if let Some(index) = voting_power_index {
                    self.dao.voting_power.remove(index);
                }
            }
        }

        #[ink(message)]
        pub fn create_proposal(&mut self) -> u128 {
            let caller = self.env().caller();

            // Check if the caller is a DAO member
            if self.dao.voting_power.iter().any(|(account, _)| *account == caller) {
                let proposal_id: u128 = self.proposal_counter;

                // Increment proposal counter for the next proposal
                self.proposal_counter += 1;

                let new_proposal = Proposal {
                    creator: caller,
                    for_votes: 0,
                    executed: false,
                };

                // Insert the new proposal into the `proposals` mapping
                self.proposals.insert(proposal_id, &new_proposal);

                // Return the proposal_id
                return proposal_id;
            } else {
                0
            }
        }

        // // vote for proposal
        #[ink(message)]
        pub fn vote_for_proposal(&mut self, proposal_id: u128, is_in_favor: bool) {
            let caller = self.env().caller();

            // Check if the caller is a DAO member
            if
                let Some((_, voting_power)) = self.dao.voting_power
                    .iter()
                    .find(|(account, _)| *account == caller)
            {
                // Retrieve the proposal from the `proposals` mapping
                if let Some(mut proposal) = self.proposals.get(&proposal_id) {
                    // Check if the proposal has not been executed
                    if !proposal.executed {
                        // Increment the 'for_votes' or 'against_votes' count for the proposal based on the boolean input
                        if is_in_favor {
                            proposal.for_votes += *voting_power;
                        }
                    }
                }
            }
        }

        #[ink(message)]
        pub fn execute_proposal(&mut self, proposal_id: u128) {
            let caller = self.env().caller();

            if self.dao.voting_power.iter().any(|(account, _)| *account == caller) {
                if let Some(mut proposal) = self.proposals.get(&proposal_id) {
                    if !proposal.executed {
                        if proposal.for_votes > self.dao.quorum {
                            proposal.executed = true;
                        }
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

        #[ink(message)]
        pub fn get_dao_members(&self) -> Vec<AccountId> {
            let dao_members: Vec<AccountId> = self.dao.voting_power
                .iter()
                .map(|(account, _)| *account)
                .collect();
            dao_members
        }

        #[ink(message)]
        pub fn get_dao_member_count(&self) -> u128 {
            self.dao.voting_power.len() as u128
        }
    }
}
