#![no_std]

use soroban_sdk::{contract, contractimpl, Bytes, Env};
use utils::{create_hodl_strategy, create_token_contract, hodl_strategy::{self, StrategyError}, mock_token, prelude, ADMIN_KEY, HODL_KEY, TOKEN_KEY, USER1_KEY, USER_KEY};

#[macro_use]
mod komet;
mod utils;

#[contract]
pub struct TestVault;

#[contractimpl]
impl TestVault {
    pub fn init(e: Env, hodl_hash: Bytes, token_hash: Bytes) {
        let admin = komet::address_from_bytes(&e, b"admin", false);
        let token = create_token_contract(&e, &admin, &token_hash);
        let hodl = create_hodl_strategy(&e, &token, &hodl_hash);
        let user = komet::address_from_bytes(&e, b"user", false);
        let user1 = komet::address_from_bytes(&e, b"user1", false);
    
        let token_client = mock_token::Client::new(&e, &token);
        // Mint 1,000,000,000 to user
        token_client.mint(&user, &1_000_000_000);
        // Mint 1,000,000,000 to user1
        token_client.mint(&user1, &1_000_000_000);
    
        e.storage().instance().set(&ADMIN_KEY, &admin);
        e.storage().instance().set(&HODL_KEY, &hodl);
        e.storage().instance().set(&TOKEN_KEY, &token);
        e.storage().instance().set(&USER_KEY, &user);
        e.storage().instance().set(&USER1_KEY, &user1);
    }

    pub fn test_deposit_and_withdrawal_flow(e: Env, amount: i128, amount_to_withdraw: i128) -> bool {
        let (_admin, hodl, token, user, _user1) = prelude(&e);
        let token = mock_token::Client::new(&e, &token);
        let strategy = hodl_strategy::Client::new(&e, &hodl);

        // Initial user token balance
        let balance = token.balance(&user);
    
        if balance < amount || amount < 0 || amount_to_withdraw < 0 {
            return true;
        }

        // Deposit amount of token from the user to the strategy
        strategy.deposit(&amount, &user);
    
        let balance_after_deposit = token.balance(&user);
        assert_eq!(balance_after_deposit, balance - amount);
    
        // Reading strategy balance
        let strategy_balance_after_deposit = token.balance(&strategy.address);
        assert_eq!(strategy_balance_after_deposit, amount);
    
        // Reading user balance on strategy contract
        let user_balance_on_strategy = strategy.balance(&user);
        assert_eq!(user_balance_on_strategy, amount);
    
        if amount_to_withdraw <= user_balance_on_strategy {
            // Withdrawing token from the strategy to user
            strategy.withdraw(&amount_to_withdraw, &user, &user);
        
            // Reading user balance in token
            let balance = token.balance(&user);
            assert_eq!(balance, balance_after_deposit + amount_to_withdraw);
        
            // Reading strategy balance in token
            let balance = token.balance(&strategy.address);
            assert_eq!(balance, amount - amount_to_withdraw);
        
            // Reading user balance on strategy contract
            let user_balance = strategy.balance(&user);
            assert_eq!(user_balance, amount - amount_to_withdraw);
        } else {
            // Attempt to withdraw more than the balance should fail
            let result = strategy.try_withdraw(&amount_to_withdraw, &user, &user);
            assert_eq!(result, Err(Ok(StrategyError::InsufficientBalance)));
        }

        true
    }
}
