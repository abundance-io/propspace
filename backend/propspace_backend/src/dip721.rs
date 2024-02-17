use crate::types::*;
use bincode::serialize;
use candid::{CandidType, Int, Nat, Principal};
use core::num;
use ic_cdk::{api, query, update};
use serde::{Deserialize, Serialize};
use std::{
    borrow::BorrowMut,
    cell::RefCell,
    collections::{HashMap, HashSet},
};

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
    total_spaces: u64,
    cycles: u64,
}

#[derive(CandidType, Deserialize, Clone)]
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
struct TokenMetaData {
    owner: Principal,
    is_burned: bool,
    properties: Vec<(String, GenericValue)>,
    burned_at: Option<u64>,
    burned_by: Option<Principal>,
    minted_at: u64,
    minted_by: Principal,
    space_id: u64,
    num_units: u64,
}

impl TokenMetaData {
    fn new(
        owner: Principal,
        properties: Option<Vec<(String, GenericValue)>>,
        num_units: u64,
        space_id: u64,
    ) -> Self {
        Self {
            owner,
            num_units,
            is_burned: false,
            properties: properties.unwrap_or_default(),
            burned_at: None,
            burned_by: None,
            minted_at: api::time(),
            minted_by: owner,
            space_id,
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

#[derive(CandidType, Deserialize, Default, Copy, Clone)]
struct Space {
    id: u64,
    price_per_unit: u64,
    num_units_available: u64,
}

type CanisterResult<T = ()> = Result<T, String>;
pub type NftResult<T = ()> = Result<T, NftError>;

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
        owner: Principal,
        space_id: u64,
        space_details: SpaceDetails,
        properties: Option<Vec<(String, GenericValue)>>,
        token_data: u64,
        num_units: u64,
    ) -> Result<(), DaoServiceError> {
        let token_data = TokenData {
            bytes: serialize(&space_details).unwrap(),
            data_type: DataType::Raw,
        };

        let minted_token_result: Result<(NftResult,), _> = ic_cdk::call(
            self.principal,
            "mintHouse",
            (owner, properties, space_id, token_data, num_units),
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

    pub async fn trade_units(
        &self,
        token_id: u64,
        sender: Principal,
        receiver: Principal,
        num_units: u64,
    ) -> Result<(), DaoServiceError> {
        let minted_token_result: Result<(NftResult<TokenIdentifier>,), _> = ic_cdk::call(
            self.principal,
            "tradeUnits",
            (token_id, sender, receiver, num_units),
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
