use crate::constants::*;
use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct UserState {
    pub user: Pubkey,
    pub invested_amount: u64,
    pub approved_amount: u64,
    pub total_withdraw_amount: u64,

    pub bump: u8,
    pub is_first: u8,

    pub withdraw_start: u64,
    pub claim_start: u64,

    pub referrer: Pubkey,
    pub referral_reward: u64,
    pub referred_count: u64,

    pub matrix_count: u8,

    pub reserves: [u64; 12],
}
