use soroban_sdk::{vec as sorobanvec, Address, Vec};

use crate::test::{create_strategy_params, defindex_vault::AssetAllocation, DeFindexVaultTest};

#[test]
fn test_withdraw_success() {
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let strategy_params = create_strategy_params(&test);
    let assets: Vec<AssetAllocation> = sorobanvec![
        &test.env,
        AssetAllocation {
            address: test.token0.address.clone(),
            ratio: 1,
            strategies: strategy_params.clone()
        }
    ];

    test.defindex_contract.initialize(
        &assets,
        &test.manager,
        &test.emergency_manager,
        &test.fee_receiver,
        &test.defindex_receiver,
        &test.defindex_factory,
    );
    let amount = 1000i128;
    
    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);
    
    test.token0_admin_client.mint(&users[0], &amount);
    let user_balance = test.token0.balance(&users[0]);
    assert_eq!(user_balance, amount);
    // here youll need to create a client for a token with the same address

    let df_balance = test.defindex_contract.user_balance(&users[0]);
    assert_eq!(df_balance, 0i128);

    test.defindex_contract.deposit(&sorobanvec![&test.env, amount], &sorobanvec![&test.env, amount], &users[0]);

    let df_balance = test.defindex_contract.user_balance(&users[0]);
    assert_eq!(df_balance, amount);

    test.defindex_contract.withdraw(&df_balance, &users[0]);
    
    let df_balance = test.defindex_contract.user_balance(&users[0]);
    assert_eq!(df_balance, 0i128);

    let user_balance = test.token0.balance(&users[0]);
    assert_eq!(user_balance, amount);
}