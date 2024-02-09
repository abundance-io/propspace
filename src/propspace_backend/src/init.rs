use crate::env::CanisterEnvironment;
use crate::service::HousingDaoService;
use crate::types::HousingDaoStorage;
use crate::SERVICE;
use ic_cdk_macros::init;

#[init]
fn init(init_state: HousingDaoStorage) {
    ic_cdk::setup();

    let mut init_service = HousingDaoService::from(init_state);
    init_service.env = Box::new(CanisterEnvironment {});

    SERVICE.with(|service| *service.borrow_mut() = init_service);
}
