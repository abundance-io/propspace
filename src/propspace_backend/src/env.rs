use ic_cdk::export::Principal;

//all canister-like environments should implement this trait
pub trait Environment {
    fn now(&self) -> u64;

    //id of canister calling the environment
    fn caller(&self) -> Principal;

    //id of the canister the environment is in
    fn canister_id(&self) -> Principal;
}

pub struct CanisterEnvironment {}
pub struct EmptyEnvironment {}

impl Environment for EmptyEnvironment {
    fn now(&self) -> u64 {
        unimplemented!()
    }

    fn caller(&self) -> Principal {
        unimplemented!()
    }

    fn canister_id(&self) -> Principal {
        unimplemented!()
    }
}

impl Environment for CanisterEnvironment {
    fn now(&self) -> u64 {
        return ic_cdk::api::time();
    }

    fn caller(&self) -> Principal {
        return ic_cdk::api::caller();
    }

    fn canister_id(&self) -> Principal {
        return ic_cdk::api::id();
    }
}

//environment to test locally
#[cfg(test)]
pub struct TestEnvironment {
    pub now: u64,
    pub caller: Principal,
    pub canister_id: Principal,
}

#[cfg(test)]
impl Environment for TestEnvironment {
    fn caller(&self) -> Principal {
        return self.caller;
    }

    fn canister_id(&self) -> Principal {
        return self.canister_id;
    }

    fn now(&self) -> u64 {
        return self.now;
    }
}
