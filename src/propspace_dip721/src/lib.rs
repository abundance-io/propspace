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
type NftResult<T = ()> = Result<T, NftError>;
type Owners = HashMap<Principal, u64>;

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
// =============== CANISTER API ================
#[query(name = "getCanisterMetadata")]
fn get_canister_metadata() -> CanisterMetaData {
    STATE.with(|state| state.borrow().canister_metadata.clone())
}

#[query(name = "getCanisterStats")]
fn get_canister_stats() -> Stats {
    STATE.with_borrow(|state| state.stats.clone())
}

#[query(name = "getCanisterLogo")]
fn get_canister_logo() -> Option<String> {
    STATE.with_borrow(|state| state.canister_metadata.logo.clone())
}

#[update(name = "setCanisterLogo", guard = "is_custodian")]
fn set_canister_logo(logo: Option<String>) {
    STATE.with_borrow_mut(|state| {
        state.canister_metadata.logo = logo;
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

#[query(name = "getCanisterSymbol")]
fn get_canister_symbol() -> Option<String> {
    STATE.with_borrow(|state| state.canister_metadata.name.clone())
}

#[update(name = "setCanisterSymbol", guard = "is_custodian")]
fn set_canister_symbol(symbol: Option<String>) {
    STATE.with_borrow_mut(|state| {
        state.canister_metadata.symbol = symbol;
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

// ============= TOKEN HANDLERS ===============

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
fn get_token_owners(token_id: u64) -> NftResult<Owners> {
    STATE.with(|state| {
        if let Some(nft) = state.borrow().tokens.get(&token_id) {
            Ok(nft.metadata.owners.clone())
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
    user: Principal,
    token_id: Option<TokenIdentifier>,
    token_data: TokenData,
    properties: Option<Vec<(String, GenericValue)>>,
    price_per_unit: u64,
    total_num_units: u64,
    num_units: u64,
) -> NftResult<TokenIdentifier> {
    STATE.with_borrow_mut(|state| {
        let token_id = token_id.unwrap_or(state.stats.total_supply + 1);
        if state.tokens.contains_key(&token_id) {
            return Err(NftError::ExistedNFT);
        }
        let stakeholders = HashMap::default();
        let token_metadata = TokenMetaData::new(
            user,
            stakeholders,
            properties,
            price_per_unit,
            total_num_units,
            num_units,
        );

        let token = Token {
            metadata: token_metadata,
            data: token_data,
        };

        state.tokens.insert(token_id, token);

        if let Some(tokens) = state.owners.get_mut(&user) {
            tokens.insert(token_id);
        } else {
            state.owners.insert(user, HashSet::from([token_id]));
            state.stats.total_unique_holders += 1;
        }

        state.stats.total_supply += 1;

        Ok(token_id)
    })
}

#[update(name = "addStakeholder", guard = "is_custodian")]
fn add_stakeholder(token_id: TokenIdentifier, stakeholder: Principal, num_units: u64) -> NftResult {
    STATE.with_borrow_mut(|state| {
        if let Some(token) = state.tokens.get_mut(&token_id) {
            let num_units_available =
                token.metadata.total_num_units - token.metadata.num_units_taken;
            if (num_units < num_units_available) {
                token
                    .metadata
                    .owners
                    .entry(stakeholder)
                    .and_modify(|e| *e += num_units)
                    .or_insert(num_units);
                token.metadata.num_units_taken += num_units;
                Ok(())
            } else {
                Err(NftError::UnitsNotAvailable)
            }
        } else {
            Err(NftError::TokenNotFound)
        }
    })
}

#[update(name = "tradeUnits", guard = "is_custodian")]
fn trade_units(
    token_id: TokenIdentifier,
    sender: Principal,
    receiever: Principal,
    num_units: u64,
) -> NftResult {
    STATE.with_borrow_mut(|state| {
        if let Some(token) = state.tokens.get_mut(&token_id) {
            if let Some(sender_tokens) = token.metadata.owners.get_mut(&sender) {
                if (*sender_tokens >= num_units) {
                    token
                        .metadata
                        .owners
                        .entry(receiever)
                        .and_modify(|e| *e += num_units)
                        .or_insert(num_units);

                    token
                        .metadata
                        .owners
                        .entry(sender)
                        .and_modify(|e| *e -= num_units)
                        .or_insert(num_units);
                    Ok(())
                } else {
                    Err(NftError::InsufficientUnits)
                }
            } else {
                Err(NftError::OperatorNotFound)
            }
        } else {
            Err(NftError::TokenNotFound)
        }
    })
}

#[query(name = "isStakeholder")]
pub fn is_stakeholder(token_id: TokenIdentifier, user: Principal) -> NftResult<bool> {
    STATE.with(|state| {
        if let Some(token) = state.borrow().tokens.get(&token_id) {
            Ok(token.metadata.owners.contains_key(&user))
        } else {
            Err(NftError::TokenNotFound)
        }
    })
}

#[query(name = "getHouseData")]
fn get_token_data(token_id: TokenIdentifier) -> NftResult<TokenData> {
    STATE.with(|state| {
        if let Some(token) = state.borrow().tokens.get(&token_id) {
            Ok(token.data.clone())
        } else {
            Err(NftError::TokenNotFound)
        }
    })
}

#[query(name = "getUserHousingUnits")]
fn get_housing_units(user: Principal) {
    STATE.with(|state| {
        let units: Vec<HousingUnit> = state
            .borrow()
            .tokens
            .iter()
            .filter_map(|(token_id, token)| match token.metadata.owners.get(&user) {
                Some(num_units) => Some(HousingUnit {
                    id: *token_id,
                    num_units: *num_units,
                }),
                None => None,
            })
            .collect();
    })
}

#[update(name = "burnToken", guard = "is_custodian")]
fn burn_token(token_id: TokenIdentifier) -> NftResult {
    STATE.with_borrow_mut(|state| {
        if let Some(token) = state.tokens.get_mut(&token_id) {
            token.metadata.owners = HashMap::default();
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
