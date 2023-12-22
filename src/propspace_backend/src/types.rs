use crate::env::{EmptyEnvironment, Environment};
use std::collections::HashMap;

use ic_cdk::export::{
    candid::{CandidType, Deserialize},
    Principal,
};

type HouseTokenIdentifier = u64;

#[derive(Clone, Copy, Debug, Default, CandidType, Deserialize)]
pub struct Tokens {
    pub amount_e8s: u64,
}
#[derive(Clone, Copy, Debug, Default, CandidType, Deserialize)]
pub struct AccountData {
    tokens: Tokens,
    housing_units: HousingUnit,
}

#[derive(Clone, Copy, Debug, Default, CandidType, Deserialize)]
pub struct HousingUnit {
    // house_identifier
    id: HouseTokenIdentifier,
    num_units: u64,
}

pub struct Proposal {
    pub id: u64,
    pub timestamp: u64,
    pub housing_unit: HouseTokenIdentifier,
    pub proposer: Principal,
    pub proposition: String,
    pub state: ProposalState,
    pub percentage_for: f64,
    pub percentage_against: f64,
    pub percentage_abstain: f64,
    pub voters: Vec<Principal>,
}

//struct to preserve and export dao data - to allow easy reuse
pub struct HousingDaoStorage {
    pub accounts: HashMap<Principal, AccountData>,
    pub proposals: HashMap<u64, Proposal>,
}

pub enum ProposalState {
    Open,
    Accepted,
    Rejected,
    Executing,
    Succeeded,
    Failed(String),
}

pub enum Proposition {
    UnitsSale(UnitSaleProposition),
    SetPrice(SetPriceProposition),
    Other(String),
}

pub struct UnitSaleProposition {
    num_units: u64,
    buyer_account: Principal,
}
pub struct SetPriceProposition {
    new_price: u64,
}
