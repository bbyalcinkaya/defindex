
use soroban_sdk::{symbol_short, Address, Bytes, Env, String, Symbol};
use soroban_sdk::{vec, Val, Vec};

use crate::komet;


pub mod hodl_strategy {
    soroban_sdk::contractimport!(
        file = "../target/wasm32-unknown-unknown/release/hodl_strategy.optimized.wasm"
    );
}

const HODL_ADDRESS: &[u8; 32] = b"hodl_contract___________________";

pub mod mock_token {
    soroban_sdk::contractimport!(
        file = "wasm/mock_sep_41_token.wasm"
    );
}

const TOKEN_ADDRESS: &[u8; 32] = b"token_contract__________________";

// Base Strategy Contract
pub fn create_hodl_strategy<'a>(e: &Env, asset: &Address, hodl_hash: &Bytes) -> Address {
    let init_args: Vec<Val> = vec![e];
    let constructor_args: Vec<Val> = vec![e, *asset.as_val(), *init_args.as_val()];

    let hodl = komet::create_contract(e, HODL_ADDRESS, hodl_hash.as_object());

    let _: () = e.invoke_contract(&hodl, &Symbol::new(e, "__constructor"), constructor_args);

    hodl
}

// Create Test Token
pub(crate) fn create_token_contract<'a>(e: &Env, admin: &Address, token_hash: &Bytes) -> Address {
    let token = komet::create_contract(e, TOKEN_ADDRESS, token_hash.as_object());
    let token_client = mock_token::Client::new(e, &token);
    token_client
        .initialize(
            admin, 
            &7, 
            &String::from_str(e, "TOKEN"),
            &String::from_str(e, "TOKEN")
        );
    token
}

pub const ADMIN_KEY: Symbol = symbol_short!("admin");
pub const TOKEN_KEY: Symbol = symbol_short!("token");
pub const HODL_KEY: Symbol = symbol_short!("hodl");
pub const USER_KEY: Symbol = symbol_short!("user");
pub const USER1_KEY: Symbol = symbol_short!("user1");

pub fn prelude(e: &Env) -> (Address, Address, Address, Address, Address) {
    (
        e.storage().instance().get(&ADMIN_KEY).unwrap(),
        e.storage().instance().get(&HODL_KEY).unwrap(),
        e.storage().instance().get(&TOKEN_KEY).unwrap(),
        e.storage().instance().get(&USER_KEY).unwrap(),
        e.storage().instance().get(&USER1_KEY).unwrap(),
    )
}