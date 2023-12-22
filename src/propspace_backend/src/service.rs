use crate::dip721::DIPService;
use crate::env::{EmptyEnvironment, Environment};
use crate::types::*;
use ic_cdk::export::Principal;
use std::collections::HashMap;

pub struct HousingDaoService {
    pub env: Box<dyn Environment>,
    pub dip_service: DIPService,
    pub accounts: HashMap<Principal, AccountData>,
    pub proposals: HashMap<u64, Proposal>,
}

impl Default for HousingDaoService {
    fn default() -> Self {
        HousingDaoService {
            env: Box::new(EmptyEnvironment {}),
            dip_service: DIPService::default(),
            accounts: HashMap::default(),
            proposals: HashMap::default(),
        }
    }
}

impl From<HousingDaoStorage> for HousingDaoService {
    fn from(storage: HousingDaoStorage) -> Self {
        HousingDaoService {
            env: Box::new(EmptyEnvironment {}),
            dip_service: DIPService::default(),
            accounts: storage.accounts,
            proposals: storage.proposals,
        }
    }
}

impl HousingDaoService {
    pub fn submit_proposal(&self, proposer: Principal, proposal: Proposal) {
        let is_stakeholder = ic_cdk::call(self.dip_service.id, "isStakeholder", ());
    }
}
