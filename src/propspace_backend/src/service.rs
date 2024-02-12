use crate::dip721::DIPService;
use crate::env::{EmptyEnvironment, Environment};
use crate::types::*;
use ic_cdk::export::Principal;
use std::collections::HashMap;

pub struct HousingDaoService {
    pub env: Box<dyn Environment>,
    pub dip_service: DIPService,
    pub accounts: HashMap<Principal, Account>,
    pub proposals: HashMap<u64, Proposal>,
    pub spaces: HashMap<u64, Space>,
    pub secret_key: String,
    pub next_space_id: u64,
}

impl Default for HousingDaoService {
    fn default() -> Self {
        HousingDaoService {
            env: Box::new(EmptyEnvironment {}),
            dip_service: DIPService::default(),
            accounts: HashMap::new(),
            proposals: HashMap::new(),
            spaces: HashMap::new(),
            secret_key: String::from("default_key"),
            next_space_id: 0,
        }
    }
}

impl From<HousingDaoStorage> for HousingDaoService {
    fn from(storage: HousingDaoStorage) -> Self {
        let accounts = storage
            .accounts
            .clone()
            .into_iter()
            .map(|a| (a.principal, a))
            .collect();

        let proposals = storage
            .proposals
            .clone()
            .into_iter()
            .map(|a| (a.id, a))
            .collect();

        let spaces = storage
            .spaces
            .clone()
            .into_iter()
            .map(|a| (a.id, a))
            .collect();

        HousingDaoService {
            env: Box::new(EmptyEnvironment {}),
            dip_service: DIPService::default(),
            accounts: accounts,
            proposals: proposals,
            spaces: spaces,
            secret_key: storage.secret_key,
            next_space_id: 0,
        }
    }
}

impl HousingDaoService {
    pub fn create_account(
        &mut self,
        secret_key: String,
        account: Account,
    ) -> Result<Principal, DaoServiceError> {
        if secret_key == self.secret_key {
            self.accounts.insert(account.principal, account.clone());
            return Ok(account.principal);
        } else {
            return Err(DaoServiceError {
                error_type: ErrorType::Failure(String::from("unable to create space")),
            });
        }
    }

    pub fn get_all_spaces(&self) -> Vec<Space> {
        self.spaces.values().cloned().collect()
    }

    pub fn get_account_details(&self) -> Result<Account, DaoServiceError> {
        let caller = self.env.caller();
        match self.accounts.get(&caller) {
            Some(account) => return Ok(account.clone()),
            None => {
                return Err(DaoServiceError {
                    error_type: ErrorType::NotFound(String::from("account not found")),
                })
            }
        };
    }

    pub fn create_space(
        &mut self,
        mut space_details: SpaceDetails,
    ) -> Result<u64, DaoServiceError> {
        let caller = self.env.caller();
        space_details.owner = caller;
        let space = Space {
            id: self.next_space_id,
            details: space_details,
        };

        self.next_space_id += 1;
        match self.spaces.insert(space.id, space) {
            Some(space) => return Ok(space.id),
            None => {
                return Err(DaoServiceError {
                    error_type: ErrorType::Failure(String::from("unable to add space")),
                })
            }
        }
    }

    pub fn list_accounts(&self) -> Vec<Account> {
        self.accounts.values().cloned().collect()
    }

    pub fn get_balance(&self) -> Result<Tokens, DaoServiceError> {
        let caller = self.env.caller();
        match self.accounts.get(&caller) {
            Some(account) => return Ok(account.tokens),
            None => {
                return Err(DaoServiceError {
                    error_type: ErrorType::NotFound(String::from("account not found")),
                })
            }
        }
    }

    pub fn get_housing_units(&self) -> Result<Vec<HousingUnit>, DaoServiceError> {
        let caller = self.env.caller();
        match self.accounts.get(&caller) {
            Some(account) => return Ok(account.housing_units.clone()),
            None => {
                return Err(DaoServiceError {
                    error_type: ErrorType::NotFound(String::from("account not found")),
                })
            }
        }
    }

    pub fn get_housing_units_from_space(
        &self,
        space_id: u64,
    ) -> Result<Vec<HousingUnit>, DaoServiceError> {
        let caller = self.env.caller();
        match self.accounts.get(&caller) {
            Some(account) => Ok(account
                .housing_units
                .clone()
                .into_iter()
                .filter(|x| space_id == x.id)
                .collect()),
            None => {
                return Err(DaoServiceError {
                    error_type: ErrorType::NotFound(String::from("account not found")),
                })
            }
        }
    }
}
