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
#[derive(Clone, Copy, Debug, CandidType, Deserialize)]
pub struct Account {
    pub principal: Principal,
    pub tokens: Tokens,
    pub housing_units: HousingUnit,
}

#[derive(Clone, Copy, Debug, Default, CandidType, Deserialize)]
pub struct HousingUnit {
    // house_identifier
    pub id: HouseTokenIdentifier,
    pub num_units: u64,
}

#[derive(Clone, Debug, CandidType, Deserialize, PartialEq, PartialOrd)]
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

#[derive(Clone, Debug, CandidType, Deserialize, PartialEq, PartialOrd)]
pub struct Space {
    pub id: u64,
    location: String,
    description: String,
    price_per_unit: u64,
    units_available: u64,
}

//struct to preserve and export dao data - to allow easy reuse
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct HousingDaoStorage {
    pub accounts: Vec<Account>,
    pub proposals: Vec<Proposal>,
    pub spaces: Vec<Space>,
}

#[derive(Clone, Debug, CandidType, Deserialize, PartialEq, PartialOrd)]
pub enum ProposalState {
    Open,
    Accepted,
    Rejected,
    Executing,
    Succeeded,
    Failed(String),
}

#[derive(Clone, Debug, CandidType, Deserialize, PartialEq, PartialOrd)]
pub enum Proposition {
    UnitsSale(UnitSaleProposition),
    SetPrice(SetPriceProposition),
    Other(String),
}

#[derive(Clone, Debug, CandidType, Deserialize, PartialEq, PartialOrd)]
pub struct UnitSaleProposition {
    num_units: u64,
    buyer_account: Principal,
}

#[derive(Clone, Debug, CandidType, Deserialize, PartialEq, PartialOrd)]
pub struct SetPriceProposition {
    new_price: u64,
}
