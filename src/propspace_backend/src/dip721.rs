use crate::types::*;
use bincode::serialize;
use core::num;
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
};

use candid::{CandidType, Int, Nat, Principal};
use ic_cdk::{api, query, update};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize)]
enum CanisterError {
    NotCustodian,
}

impl Into<String> for CanisterError {
    fn into(self) -> String {
        match self {
            CanisterError::NotCustodian => "You are not a custodian in this canister".to_owned(),
        }
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub enum NftError {
    SelfTransfer,
    TokenNotFound,
    TxNotFound,
    SelfApprove,
    OperatorNotFound,
    UnauthorizedOwner,
    UnauthorizedOperator,
    ExistedNFT,
    OwnerNotFound,
    UnitsNotAvailable,
    InsufficientUnits,
    SenderNotOwner,
    Other(String),
}

#[derive(CandidType, Deserialize, Clone)]
enum DataType {
    Link,
    Raw,
}

#[derive(CandidType, Deserialize, Clone, Default)]
struct CanisterMetaData {
    name: Option<String>,
    symbol: Option<String>,
    logo: Option<String>,
    created_at: u64,
    upgraded_at: u64,
    custodians: HashSet<Principal>,
}

#[derive(CandidType, Deserialize, Default, Clone)]
struct Stats {
    total_supply: u64,
    total_transactions: u64,
    total_unique_holders: u64,
    cycles: u64,
}

#[derive(CandidType, Deserialize)]
struct Token {
    metadata: TokenMetaData,
    data: TokenData,
}

// ! Need to check on what the blob type in Candid is in Rust
type Bytes = Vec<u8>;

#[derive(CandidType, Deserialize, Clone)]
struct TokenData {
    bytes: Bytes,
    data_type: DataType,
}

type TokenIdentifier = u64;

#[derive(CandidType, Deserialize, Serialize, Clone)]
pub enum GenericValue {
    BoolContent(bool),
    TextContent(String),
    BlobContent(Vec<u8>),
    Principal(Principal),
    Nat8Content(u8),
    Nat16Content(u16),
    Nat32Content(u32),
    Nat64Content(u64),
    NatContent(Nat),
    Int8Content(i8),
    Int16Content(i16),
    Int32Content(i32),
    Int64Content(i64),
    IntContent(Int),
    FloatContent(f64), // motoko only support f64
    NestedContent(Vec<(String, GenericValue)>),
}

#[derive(CandidType, Serialize, Deserialize, Clone)]
struct HousingUnit {
    id: TokenIdentifier,
    num_units: u64,
}
#[derive(CandidType, Serialize, Deserialize, Clone)]
struct TokenMetaData {
    owners: Owners,
    is_burned: bool,
    properties: Vec<(String, GenericValue)>,
    burned_at: Option<u64>,
    burned_by: Option<Principal>,
    minted_at: u64,
    minted_by: Principal,
    //prospace specific additions
    price_per_unit: u64,
    total_num_units: u64,
    num_units_taken: u64,
}

impl TokenMetaData {
    fn new(
        owner: Principal,
        owners: Owners,
        properties: Option<Vec<(String, GenericValue)>>,
        price_per_unit: u64,
        total_num_units: u64,
        num_units: u64,
    ) -> Self {
        Self {
            owners,
            is_burned: false,
            properties: properties.unwrap_or_default(),
            burned_at: None,
            burned_by: None,
            minted_at: api::time(),
            num_units_taken: 0,
            minted_by: owner,
            price_per_unit,
            total_num_units,
        }
    }
}

#[derive(CandidType, Deserialize)]
struct InitArguments {
    name: Option<String>,
    symbol: Option<String>,
    logo: Option<String>,
    custoidians: Option<HashSet<Principal>>,
}

#[derive(CandidType, Deserialize, Default)]
struct State {
    canister_metadata: CanisterMetaData,
    tokens: HashMap<TokenIdentifier, Token>,
    owners: HashMap<Principal, HashSet<TokenIdentifier>>,
    stats: Stats,
}

type CanisterResult<T = ()> = Result<T, String>;

pub type NftResult<T = ()> = Result<T, NftError>;
type Owners = HashMap<Principal, u64>;

pub struct DIP721Service {
    principal: Principal,
}

impl From<Principal> for DIP721Service {
    fn from(principal: Principal) -> Self {
        Self { principal }
    }
}

impl DIP721Service {
    pub async fn mint_token(
        &self,
        user: Principal,
        token_id: Option<TokenIdentifier>,
        space_details: SpaceDetails,
        properties: Option<Vec<(String, GenericValue)>>,
        price_per_unit: u64,
        total_num_units: u64,
        num_units: u64,
    ) -> Result<(), DaoServiceError> {
        let token_data = TokenData {
            bytes: serialize(&space_details).unwrap(),
            data_type: DataType::Raw,
        };

        let minted_token_result: Result<(NftResult<TokenIdentifier>,), _> = ic_cdk::call(
            self.principal,
            "mintHouse",
            (
                user,
                token_id,
                token_data,
                properties,
                space_details.price_per_unit,
                space_details.total_num_units,
                num_units,
            ),
        )
        .await;

        match (minted_token_result) {
            Ok(res) => match res.0 {
                Ok(id) => return Ok(()),

                Err(err) => Err(DaoServiceError {
                    error_type: ErrorType::NftError(err),
                }),
            },

            Err(err) => Err(DaoServiceError {
                error_type: ErrorType::CanisterError(err.1),
            }),
        }
    }
}
