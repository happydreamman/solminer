use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Settings {
    pub admin: Pubkey,
    pub dev_wallet: Pubkey,
    pub marketing_wallet: Pubkey,
    pub pool: Pubkey,

    pub roi: u64,
    pub fee: u64,
    pub compound_fee: u64,
    pub withdraw_tax_rate: u64,
    pub ref_fee: u64,

    pub last_deposit_user: Pubkey,
    pub last_deposit_time: u64,
    pub pool_prize_limit: u64,
    pub pool_prize_ratio: u64,
    
    pub members: u64,
    
    pub bump: u8,
    pub pool_bump: u8,
    pub miner_started: u8,

    pub last_pool_winner: Pubkey,
    pub last_pool_reward: u64,
    pub reserves: [u64; 6],
}
