use crate::types::*;
use candid::Principal;
use ic_cdk;
use ic_cdk::export::candid::candid_method;
use ic_cdk::export::candid::export_service;
use service::HousingDaoService;

use std::cell::RefCell;

mod dip721;
mod env;
mod init;
mod service;
mod types;

thread_local! {
    static SERVICE: RefCell<HousingDaoService> = RefCell::default();
}

#[ic_cdk::update]
#[ic_cdk::export::candid::candid_method]
fn list_spaces() -> Vec<Space> {
    SERVICE.with(|service| service.borrow_mut().get_all_spaces())
}

#[ic_cdk::query]
#[ic_cdk::export::candid::candid_method]
fn get_housing_units() -> Result<Vec<HousingUnit>, DaoServiceError> {
    SERVICE.with(|service| service.borrow_mut().get_housing_units())
}

#[ic_cdk::query]
#[ic_cdk::export::candid::candid_method]
fn get_housing_units_from_space(space_id: u64) -> Result<Vec<HousingUnit>, DaoServiceError> {
    SERVICE.with(|service| service.borrow_mut().get_housing_units_from_space(space_id))
}
#[ic_cdk::update]
#[ic_cdk::export::candid::candid_method]
fn create_account(account: Account, secret_key: String) -> Result<Principal, DaoServiceError> {
    SERVICE.with(|service| service.borrow_mut().create_account(secret_key, account))
}

#[ic_cdk::update]
#[ic_cdk::export::candid::candid_method]
fn create_space(details: SpaceDetails) -> Result<u64, DaoServiceError> {
    SERVICE.with(|service| service.borrow_mut().create_space(details))
}

#[ic_cdk::query]
#[ic_cdk::export::candid::candid_method(query)]
fn list_accounts() -> Vec<Account> {
    SERVICE.with(|service| service.borrow().list_accounts())
}

#[ic_cdk::query]
#[ic_cdk::export::candid::candid_method(query)]
fn get_account_details() -> Result<Account, DaoServiceError> {
    SERVICE.with(|service| service.borrow().get_account_details())
}

#[ic_cdk::query]
#[ic_cdk::export::candid::candid_method(query)]
fn get_balance() -> Result<Tokens, DaoServiceError> {
    SERVICE.with(|service| service.borrow().get_balance())
}

ic_cdk::export::candid::export_service!();

#[ic_cdk::query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    __export_service()
}

#[cfg(test)]
mod tests {
    use super::export_candid;
    #[test]
    fn save_candid() {
        use std::env;
        use std::fs::write;
        use std::path::PathBuf;
        let dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        let dir = dir.parent().unwrap();
        write(
            dir.join("./propspace_backend/propspace_backend.did"),
            export_candid(),
        )
        .expect("Write failed.");
    }
}
