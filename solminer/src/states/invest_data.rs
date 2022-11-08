use crate::constants::*;
use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct InvestData {
    pub user: Pubkey,
    pub seed_key: Pubkey,
    pub amount: u64,
    pub invest_time: u64,
    pub days: u64,
    pub active_balance: u64,

    pub last_roi_time: u64,
    pub bump: u8,
    pub reserves: [u64; 2],
}
