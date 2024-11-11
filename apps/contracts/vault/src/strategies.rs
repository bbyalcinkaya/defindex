use defindex_strategy_core::DeFindexStrategyClient;
use soroban_sdk::{Address, Env, vec, IntoVal, Symbol};
use soroban_sdk::auth::{ContractContext, InvokerContractAuthEntry, SubContractInvocation};


use crate::{
    models::{AssetStrategySet, Strategy},
    storage::{get_asset, get_assets, get_total_assets, set_asset},
    ContractError,
};

pub fn get_strategy_client(e: &Env, address: Address) -> DeFindexStrategyClient {
    DeFindexStrategyClient::new(&e, &address)
}

/// Finds the asset corresponding to the given strategy address.
pub fn get_strategy_asset(
    e: &Env,
    strategy_address: &Address,
) -> Result<AssetStrategySet, ContractError> {
    let assets = get_assets(e);

    for asset in assets.iter() {
        if asset
            .strategies
            .iter()
            .any(|strategy| &strategy.address == strategy_address)
        {
            return Ok(asset);
        }
    }

    Err(ContractError::StrategyNotFound)
}

/// Finds the AssetStrategySet corresponding to the given asset address.
pub fn get_asset_allocation_from_address(
    e: &Env,
    asset_address: Address,
) -> Result<AssetStrategySet, ContractError> {
    let assets = get_assets(e);

    for asset in assets.iter() {
        if asset.address == asset_address {
            return Ok(asset);
        }
    }

    Err(ContractError::AssetNotFound)
}

/// Finds the strategy struct corresponding to the given strategy address within the given asset.
pub fn get_strategy_struct(
    strategy_address: &Address,
    asset: &AssetStrategySet,
) -> Result<Strategy, ContractError> {
    asset
        .strategies
        .iter()
        .find(|strategy| &strategy.address == strategy_address && !strategy.paused)
        .ok_or(ContractError::StrategyNotFound)
}

/// Pauses a strategy by setting its `paused` field to `true`.
/// Finds the asset that contains the strategy and updates the storage.
pub fn pause_strategy(e: &Env, strategy_address: Address) -> Result<(), ContractError> {
    let total_assets = get_total_assets(e);

    // Iterate through all assets to find the one that contains the strategy
    for i in 0..total_assets {
        let mut asset = get_asset(e, i);

        // Check if this asset contains the strategy
        for (j, strategy) in asset.strategies.iter().enumerate() {
            if strategy.address == strategy_address {
                // Pause the strategy by modifying its `paused` field
                let mut updated_strategy = strategy.clone();
                updated_strategy.paused = true;

                // Update the strategy in the asset
                asset.strategies.set(j as u32, updated_strategy);

                // Save the updated asset back into storage
                set_asset(e, i, &asset);

                return Ok(());
            }
        }
    }

    // If no strategy is found, return an error
    Err(ContractError::StrategyNotFound)
}

/// Unpauses a strategy by setting its `paused` field to `false`.
/// Finds the asset that contains the strategy and updates the storage.
pub fn unpause_strategy(e: &Env, strategy_address: Address) -> Result<(), ContractError> {
    let total_assets = get_total_assets(e);

    // Iterate through all assets to find the one that contains the strategy
    for i in 0..total_assets {
        let mut asset = get_asset(e, i);

        // Check if this asset contains the strategy
        for (j, strategy) in asset.strategies.iter().enumerate() {
            if strategy.address == strategy_address {
                // Unpause the strategy by modifying its `paused` field
                let mut updated_strategy = strategy.clone();
                updated_strategy.paused = false;

                // Update the strategy in the asset
                asset.strategies.set(j as u32, updated_strategy);

                // Save the updated asset back into storage
                set_asset(e, i, &asset);

                return Ok(());
            }
        }
    }

    // If no strategy is found, return an error
    Err(ContractError::StrategyNotFound)
}

pub fn withdraw_from_strategy(
    e: &Env,
    strategy_address: &Address,
    amount: &i128,
) -> Result<(), ContractError> {
    let strategy_client = get_strategy_client(e, strategy_address.clone());

    match strategy_client.try_withdraw(amount, &e.current_contract_address()) {
        Ok(Ok(_)) => Ok(()),
        Ok(Err(_)) | Err(_) => Err(ContractError::StrategyWithdrawError),
    }
}

pub fn invest_in_strategy(
    e: &Env,
    asset_address: &Address,
    strategy_address: &Address,
    amount: &i128,
) -> Result<(), ContractError> {
    

    // // Now we will handle funds on behalf of the contract, not the caller (manager or user)
    // let mut transfer_args: Vec<Val> = vec![&e];
    // transfer_args.push_back(e.current_contract_address().into_val(&e)); //from 
    // transfer_args.push_back(strategy_address.into_val(&e)); //to 
    // transfer_args.push_back(amount.into_val(&e)); //amount


    e.authorize_as_current_contract(vec![
        &e,
        InvokerContractAuthEntry::Contract(SubContractInvocation {
            context: ContractContext {
                contract: asset_address.clone(),
                fn_name: Symbol::new(&e, "transfer"),
                args: (
                    e.current_contract_address(),
                    strategy_address,
                    amount.clone()).into_val(e),
                    
            },
            sub_invocations: vec![&e],
        }),
    ]);


    let strategy_client = get_strategy_client(&e, strategy_address.clone());

    match strategy_client.try_deposit(amount, &e.current_contract_address()) {
        Ok(Ok(_)) => Ok(()),
        Ok(Err(_)) | Err(_) => Err(ContractError::StrategyInvestError),
    }
}
