use crate::{setup::create_vault_one_asset_fixed_strategy, test::{EnvTestUtils, IntegrationTest, ONE_YEAR_IN_SECONDS}, vault::{defindex_vault_contract::{AssetInvestmentAllocation, StrategyAllocation}, VaultContractError, MINIMUM_LIQUIDITY}};
use soroban_sdk::{testutils::{Ledger, MockAuth, MockAuthInvoke}, vec as svec,  IntoVal, Vec};

extern crate std;

#[test]
fn fee_performance() {
    let enviroment = create_vault_one_asset_fixed_strategy();
    let setup = enviroment.setup;

    let user_starting_balance = 100_000_0_000_000i128;

    let users = IntegrationTest::generate_random_users(&setup.env, 1);
    let user = &users[0];

    enviroment.token_admin_client.mock_auths(&[MockAuth {
        address: &enviroment.token_admin.clone(),
        invoke: &MockAuthInvoke {
            contract: &enviroment.token.address.clone(),
            fn_name: "mint",
            args: (user, user_starting_balance,).into_val(&setup.env),
            sub_invokes: &[],
        },
    }]).mint(user, &user_starting_balance);
    let user_balance = enviroment.token.balance(user);
    assert_eq!(user_balance, user_starting_balance);

    let deposit_amount = 10_0_000_000i128;
    enviroment.vault_contract
    .mock_auths(&[MockAuth {
        address: &user.clone(),
        invoke: &MockAuthInvoke {
            contract: &enviroment.vault_contract.address.clone(),
            fn_name: "deposit",
            args: (
                Vec::from_array(&setup.env,[deposit_amount]),
                Vec::from_array(&setup.env,[deposit_amount]),
                user.clone(),
                false
            ).into_val(&setup.env),
            sub_invokes: &[
                MockAuthInvoke {
                    contract: &enviroment.token.address.clone(),
                    fn_name: "transfer",
                    args: (
                        user.clone(), 
                        &enviroment.vault_contract.address.clone(),
                        deposit_amount
                    ).into_val(&setup.env),
                    sub_invokes: &[]
                }
            ]
        },
    }])
    .deposit(&svec![&setup.env, deposit_amount], &svec![&setup.env, deposit_amount], &user, &false);

    // Create investment strategies for the deposited tokens
    let investments = svec![
        &setup.env,
        Some(AssetInvestmentAllocation {
            asset: enviroment.token.address.clone(),
            strategy_allocations: svec![
                &setup.env,
                Some(StrategyAllocation {
                    amount: deposit_amount,
                    strategy_address: enviroment.strategy_contract.address.clone(),
                }),
            ],
        }),
    ];

    enviroment.vault_contract
    .mock_auths(&[MockAuth {
        address: &enviroment.manager.clone(),
        invoke: &MockAuthInvoke {
            contract: &enviroment.vault_contract.address.clone(),
            fn_name: "invest",
            args: (
                Vec::from_array(&setup.env,[
                    Some(
                        AssetInvestmentAllocation {
                            asset: enviroment.token.address.clone(),
                            strategy_allocations:
                                svec![&setup.env,
                                    Some(StrategyAllocation {
                                        amount: deposit_amount,
                                        strategy_address: enviroment.strategy_contract.address.clone(),
                                    })
                                ]
                        }
                    )
                ]),
            ).into_val(&setup.env),
            sub_invokes: &[]
        },
    }])
    .invest(&investments);
    let vault_balance_in_strategy = enviroment.strategy_contract.balance(&enviroment.vault_contract.address);

    std::println!("Shares: {:?}", vault_balance_in_strategy);

    setup.env.jump_time(ONE_YEAR_IN_SECONDS);

    enviroment.strategy_contract.harvest(&enviroment.vault_contract.address);

    let vault_balance_in_strategy = enviroment.strategy_contract.balance(&enviroment.vault_contract.address);
    std::println!("Shares after one year: {:?}", vault_balance_in_strategy);

    let report = enviroment.vault_contract.mock_all_auths().report();
    std::println!("Report: {:?}", report);
    
    assert_eq!(vault_balance_in_strategy, (deposit_amount * 11 / 10));

    let lock_fees_bps = 2000u32;
    let lock_fees_result = enviroment.vault_contract.mock_auths(&[MockAuth {
        address: &enviroment.manager.clone(),
        invoke: &MockAuthInvoke {
            contract: &enviroment.vault_contract.address.clone(),
            fn_name: "lock_fees",
            args: svec![&setup.env, lock_fees_bps].into_val(&setup.env),
            sub_invokes: &[]
    },
    }]).lock_fees(&Some(lock_fees_bps));

    std::println!("🟡Lock fees result: {:?}", lock_fees_result);

    let report_result = enviroment.vault_contract.mock_auths(&[MockAuth {
        address: &enviroment.manager.clone(),
        invoke: &MockAuthInvoke {
            contract: &enviroment.vault_contract.address.clone(),
            fn_name: "report",
            args: (  ).into_val(&setup.env),
            sub_invokes: &[]
    },
    }]).try_report();

    std::println!("🟡Report result: {:?}", report_result);

    let release_fees_amount = 100i128;
    let release_fees_result = enviroment.vault_contract.mock_auths(&[MockAuth {
        address: &enviroment.manager.clone(),
        invoke: &MockAuthInvoke {
            contract: &enviroment.vault_contract.address.clone(),
            fn_name: "release_fees",
            args: (
                &enviroment.strategy_contract.address.clone(),
                release_fees_amount
            ).into_val(&setup.env),
            sub_invokes: &[]
    },
    }]).release_fees(&enviroment.strategy_contract.address.clone(), &release_fees_amount);

    std::println!("🟡Release fees result: {:?}", release_fees_result);
    //assert_eq!(release_fees_result, Err(Ok(VaultContractError::InsufficientManagedFunds)));
    
    let distribute_fees_result = enviroment.vault_contract.mock_auths(&[MockAuth {
        address: &enviroment.manager.clone(),
        invoke: &MockAuthInvoke {
            contract: &enviroment.vault_contract.address.clone(),
            fn_name: "distribute_fees",
            args: ().into_val(&setup.env),
            sub_invokes: &[]
    },
    }]).distribute_fees();

    std::println!("🟡Distribute fees result: {:?}", distribute_fees_result);

    let report_result = enviroment.vault_contract.mock_auths(&[MockAuth {
        address: &enviroment.manager.clone(),
        invoke: &MockAuthInvoke {
            contract: &enviroment.vault_contract.address.clone(),
            fn_name: "report",
            args: (  ).into_val(&setup.env),
            sub_invokes: &[]
    },
    }]).try_report();

    std::println!("🟡Report result: {:?}", report_result);

    /* 

- before = get_asset_amounts_per_shares(1000):

- lock_fees():
dependiendo del tiempo que pase, vamos a saber cuanto genera la estrategia.
Si la fixed APR tiene 10%, hacemos pasar un año y la estrategia debería tener un 10% más
1000

- get_locked_fees(strategy); deberia ser la ganancia

*vault_fee_rate (ex. 20%) 

- after = get_asset_amounts_per_shares(1000):
- assert (before, after*0.8)
     */

}