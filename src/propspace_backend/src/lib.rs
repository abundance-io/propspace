use ic_cdk::export::candid::candid_method;
use ic_cdk::export::candid::export_service;
mod dip721;
mod env;
mod service;
mod types;

#[candid_method]
#[ic_cdk::query]
fn submit_proposal(proposal: String) -> String {
    format!("{}!", proposal)
}

#[candid_method]
#[ic_cdk::query]
fn execute_proposal(proposal: String) -> String {
    format!("Hello, {}!", proposal)
}

#[ic_cdk::query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    export_service!();
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
        write(dir.join("propspace_backend.did"), export_candid()).expect("Write failed.");
    }
}
