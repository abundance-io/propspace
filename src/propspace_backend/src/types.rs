use crate::dip721::NftError;
use crate::env::{EmptyEnvironment, Environment};
use std::collections::HashMap;

use ic_cdk::export::{
    candid::{CandidType, Deserialize},
    Principal,
};
use serde_derive::Serialize;

#[derive(Clone, Copy, Debug, Default, CandidType, Deserialize)]
pub struct Tokens {
    pub amount_e8s: u64,
}
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Account {
    pub principal: Principal,
    pub tokens: Tokens,
    pub housing_units: Vec<HousingUnit>,
}

#[derive(Clone, Copy, Debug, Default, CandidType, Deserialize)]
pub struct HousingUnit {
    // house_identifier
    pub id: u64,
    pub num_units: u64,
}

#[derive(Clone, Debug, CandidType, Deserialize, PartialEq, PartialOrd)]
pub struct Proposal {
    pub id: u64,
    pub timestamp: u64,
    pub housing_unit: u64,
    pub proposer: Principal,
    pub proposition: String,
    pub state: ProposalState,
    pub percentage_for: f64,
    pub percentage_against: f64,
    pub percentage_abstain: f64,
    pub voters: Vec<Principal>,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct Space {
    pub id: u64,
    pub details: SpaceDetails,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct SpaceDetails {
    pub owner: Principal,
    pub location: String,
    pub description: String,
    pub price_per_unit: u64,
    pub units_available: u64,
}

//struct to preserve and export dao data - to allow easy reuse
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct HousingDaoStorage {
    pub dip_service_principal: Principal,
    pub accounts: Vec<Account>,
    pub proposals: Vec<Proposal>,
    pub spaces: Vec<Space>,
    pub secret_key: String,
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

#[derive(Clone, CandidType, Deserialize)]
pub struct DaoServiceError {
    pub error_type: ErrorType,
}

#[derive(Clone, CandidType, Deserialize)]
pub enum ErrorType {
    Unauthorized(String),
    NotFound(String),
    Failure(String),
    NftError(NftError),
    CanisterError(String),
}
