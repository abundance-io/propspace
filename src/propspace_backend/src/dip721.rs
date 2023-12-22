use ic_cdk::export::Principal;
pub struct DIPService {
    pub id: Principal,
}

impl Default for DIPService {
    fn default() -> Self {
        DIPService {
            id: Principal::from_text("").unwrap(),
        }
    }
}
