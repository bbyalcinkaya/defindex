use soroban_sdk::{contracttype, Address, Env};

use crate::{constants::MAX_BPS, storage::{get_report, set_report}};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Report {
    pub prev_balance: i128,
    pub gains_or_losses: i128,
    pub locked_fee: i128,
}

impl Report {
    pub fn lock_fee(&mut self, fee_rate: u32) -> i128 {
        let gains_or_losses = self.gains_or_losses;
        let numerator = gains_or_losses
            .checked_mul(fee_rate as i128)
            .unwrap();
        let total_fee = numerator.checked_div(MAX_BPS).unwrap();

        self.locked_fee += total_fee;
        self.gains_or_losses = 0;
        
        total_fee
    }

    pub fn report(&mut self, current_balance: i128) -> i128 {
        let prev_balance = self.prev_balance;
        
        let gains_or_losses = current_balance - prev_balance;
        self.gains_or_losses += gains_or_losses;
        self.prev_balance = current_balance;

        gains_or_losses
    }
}

/// Reports the gains or losses for a strategy based on the current balance.
///
/// # Arguments
/// * `e` - A reference to the environment.
/// * `strategy` - The address of the strategy.
/// * `current_balance` - A reference to the current balance.
///
/// # Returns
/// A tuple with the current balance and updated gains or losses.
pub fn report(e: &Env, strategy: &Address, current_balance: &i128) -> (i128, i128) {
    let mut report = get_report(e, strategy);
    
    let current_gains_or_losses = report.report(current_balance.clone());
    set_report(e, strategy, report);

    (current_balance.clone(), current_gains_or_losses)
}