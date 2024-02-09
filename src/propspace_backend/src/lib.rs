use crate::types::*;
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

#[candid_method]
#[ic_cdk::query]
fn submit_proposal(proposal: String) -> String {
    format!("{}!", proposal)
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
