use core::num;
use std::{
    borrow::BorrowMut,
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

#[derive(CandidType, Serialize)]
enum NftError {
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

#[derive(CandidType, Deserialize, Serialize)]
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

#[derive(CandidType, Deserialize, Default)]
struct State {
    canister_metadata: CanisterMetaData,
    tokens: HashMap<TokenIdentifier, Token>,
    owners: HashMap<Principal, HashSet<TokenIdentifier>>,
    spaces: HashMap<u64, Space>,
    stats: Stats,
}

type CanisterResult<T = ()> = Result<T, String>;
type NftResult<T = ()> = Result<T, NftError>;

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

#[ic_cdk::init]
fn init(args: Option<InitArguments>) {
    let default_custodians = HashSet::from([api::caller()]);
    STATE.with(|state| {
        let metadata = &mut state.borrow_mut().canister_metadata;
        if let Some(args) = args {
            metadata.name = args.name;
            metadata.symbol = args.symbol;
            metadata.logo = args.logo;
            metadata.custodians = args.custoidians.unwrap_or(default_custodians);
            metadata.created_at = api::time();
            metadata.upgraded_at = api::time();
        } else {
            metadata.custodians = default_custodians;
        }
    });
}

fn is_custodian() -> CanisterResult {
    STATE.with_borrow(|state| {
        state
            .canister_metadata
            .custodians
            .contains(&api::caller())
            .then_some(())
            .ok_or(CanisterError::NotCustodian.into())
    })
}
#[query(name = "getCanisterName")]
fn get_canister_name() -> Option<String> {
    STATE.with_borrow(|state| state.canister_metadata.name.clone())
}

#[update(name = "setCanisterName", guard = "is_custodian")]
fn set_canister_name(name: Option<String>) {
    STATE.with_borrow_mut(|state| {
        state.canister_metadata.name = name;
    })
}

#[query(name = "getCanisterCustodians")]
fn get_canister_custodians() -> HashSet<Principal> {
    STATE.with_borrow(|state| state.canister_metadata.custodians.clone())
}

#[update(name = "setCanisterCustodians", guard = "is_custodian")]
fn set_canister_custodians(custodians: HashSet<Principal>) {
    STATE.with_borrow_mut(|state| {
        state.canister_metadata.custodians = custodians;
    })
}

#[query(name = "getCanisterCycles")]
fn get_canister_cycles() -> Nat {
    STATE.with(|state| Nat::from(state.borrow().stats.cycles))
}

#[query(name = "totalUniqueHolders")]
fn get_total_unique_holders() -> Nat {
    STATE.with(|state| Nat::from(state.borrow().stats.total_unique_holders))
}

#[query(name = "getTokenMetadata")]
fn get_token_metadata(token_id: TokenIdentifier) -> NftResult<TokenMetaData> {
    STATE.with(|state| {
        if let Some(token) = state.borrow().tokens.get(&token_id) {
            Ok(token.metadata.clone())
        } else {
            Err(NftError::TokenNotFound)
        }
    })
}

#[query(name = "balanceOf")]
fn get_user_token_count(user: Principal) -> NftResult<Nat> {
    STATE.with(|state| {
        if let Some(user) = state.borrow().owners.get(&user) {
            Ok(Nat::from(user.len()))
        } else {
            Err(NftError::OwnerNotFound)
        }
    })
}

#[query(name = "ownerOf")]
fn get_token_owner(token_id: u64) -> NftResult<Principal> {
    STATE.with(|state| {
        if let Some(nft) = state.borrow().tokens.get(&token_id) {
            Ok(nft.metadata.owner)
        } else {
            Err(NftError::TokenNotFound)
        }
    })
}

#[query(name = "ownerTokenIdentifiers")]
fn get_tokens_by_owner(user: Principal) -> NftResult<HashSet<u64>> {
    STATE.with(|state| {
        if let Some(tokens) = state.borrow().owners.get(&user) {
            Ok(tokens.clone())
        } else {
            Err(NftError::OwnerNotFound)
        }
    })
}

#[query(name = "ownerTokenMetadata")]
fn get_tokens_metadata_by_owner(user: Principal) -> NftResult<Vec<TokenMetaData>> {
    STATE.with(|state| {
        let state = state.borrow();
        if let Some(tokens) = state.owners.get(&user) {
            Ok(tokens
                .iter()
                .filter_map(|token_id| state.tokens.get(token_id).map(|nft| nft.metadata.clone()))
                .collect())
        } else {
            Err(NftError::OwnerNotFound)
        }
    })
}

#[query(name = "totalSupply")]
fn get_canister_supply() -> Nat {
    STATE.with_borrow(|state| Nat::from(state.stats.total_supply))
}

#[update(name = "mintHouse", guard = "is_custodian")]
fn mint_token(
    owner: Principal,
    properties: Option<Vec<(String, GenericValue)>>,
    space_id: u64,
    token_data: TokenData,
    num_units: u64,
) -> NftResult<TokenIdentifier> {
    STATE.with_borrow_mut(|state| {
        let token_id = state.stats.total_supply + 1;
        if state.tokens.contains_key(&token_id) {
            return Err(NftError::ExistedNFT);
        }
        let token_metadata = TokenMetaData::new(owner, properties, num_units, space_id);

        let token = Token {
            metadata: token_metadata,
            data: token_data,
        };

        let space = state.spaces.get(&space_id);
        match (space) {
            Some(space) => {
                if (space.num_units_available > 0) {
                    state.tokens.insert(token_id, token);
                    state.stats.total_supply += 1;
                    let saved_owner = state.owners.get(&owner);
                    match (saved_owner) {
                        Some(_) => {}
                        None => {
                            //random state for owner bound to first ever token?
                            state.owners.insert(owner, HashSet::from([token_id]));
                        }
                    }
                } else {
                    return Err(NftError::UnitsNotAvailable);
                }
            }
            None => {
                return Err(NftError::Other(String::from("house does not exist")));
            }
        }
        Ok(token_id)
    })
}

#[update(name = "tradeUnits", guard = "is_custodian")]
fn trade_units(
    //empty string if optional "" add struct later
    token_id: TokenIdentifier,
    sender: Principal,
    receiever: Principal,
    num_units: u64,
) -> NftResult {
    STATE.with_borrow_mut(|state| {
        if let Some(token) = state.tokens.get_mut(&token_id) {
            if sender == token.metadata.owner {
                if (token.metadata.num_units >= num_units) {
                    //debit sender
                    state
                        .tokens
                        .entry(token_id)
                        .and_modify(|e| e.metadata.num_units -= num_units);

                    //fix transfer to reciever later
                    Ok(())
                } else {
                    Err(NftError::InsufficientUnits)
                }
            } else {
                Err(NftError::UnauthorizedOwner)
            }
        } else {
            Err(NftError::TokenNotFound)
        }
    })
}

#[query(name = "isOwner")]
pub fn is_owner(token_id: TokenIdentifier, user: Principal) -> NftResult<bool> {
    STATE.with(|state| {
        if let Some(token) = state.borrow().tokens.get(&token_id) {
            if (token.metadata.owner == user) {
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Err(NftError::TokenNotFound)
        }
    })
}

#[query(name = "getSpaceData")]
fn get_space_data(space_id: u64) -> NftResult<Space> {
    STATE.with(|state| {
        if let Some(space) = state.borrow().spaces.get(&space_id) {
            Ok(space.clone())
        } else {
            Err(NftError::Other(String::from("space not found")))
        }
    })
}

#[update(name = "createSpace", guard = "is_custodian")]
fn create_space(space: Space) -> NftResult<Space> {
    STATE.with_borrow_mut(|state| {
        state.spaces.insert(state.stats.total_spaces + 1, space);
        state.stats.total_spaces += 1;
        Ok(space)
    })
}

#[query(name = "getAllUserTokens")]
fn get_all_user_tokens(user: Principal) -> Vec<Token> {
    STATE.with(|state| {
        state
            .borrow()
            .tokens
            .values()
            .filter(|x| x.metadata.owner == user)
            .cloned()
            .collect()
    })
}
#[update(name = "burnToken", guard = "is_custodian")]
fn burn_token(token_id: TokenIdentifier) -> NftResult {
    STATE.with_borrow_mut(|state| {
        if let Some(token) = state.tokens.get_mut(&token_id) {
            token.metadata.owner = Principal::anonymous();
            token.metadata.is_burned = true;
            token.metadata.burned_at = Some(api::time());
            token.metadata.burned_by = Some(api::caller());
            token.metadata.minted_by = Principal::anonymous();
            state.stats.total_supply -= 1;
            Ok(())
        } else {
            Err(NftError::TokenNotFound)
        }
    })
}

ic_cdk::export_candid!();
