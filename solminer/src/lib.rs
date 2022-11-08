use anchor_lang::prelude::*;

declare_id!("JA9S2YDFqL9RbB71dmvJQjtzd2mr9tZC8psD9gYTdnbn");

/// constant
pub mod constants;
/// error
pub mod error;
/// instructions
pub mod instructions;
/// states
pub mod states;
/// utils
pub mod utils;

use crate::instructions::*;

#[program]
pub mod solminer {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }

    pub fn add_blacklist(ctx: Context<AddBlacklist>, addr: Pubkey) -> Result<()> {
        add_blacklist::handler(ctx, addr)
    }

    pub fn init_blacklist(ctx: Context<InitBlackList>) -> Result<()> {
      init_blacklist::handler(ctx)
    }

    pub fn remove_from_blacklist(ctx: Context<RemoveFromBlacklist>, addr: Pubkey) -> Result<()> {
        remove_from_blacklist::handler(ctx, addr)
    }
  
    pub fn set_pool_prize(ctx: Context<SetPoolPrize>, mins: u64, ratio: u64) -> Result<()> {
      set_pool_prize::handler(ctx, mins, ratio)
    }

    pub fn start_miner(ctx: Context<StartMiner>) -> Result<()> {
      start_miner::handler(ctx)
    }

    pub fn compound(ctx: Context<Compound>) -> Result<()> {
        compound::handler(ctx)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64, seed_key: Pubkey) -> Result<()> {
        deposit::handler(ctx, amount, seed_key)
    }

    pub fn init_user_state(ctx: Context<InitUserState>, referrer: Pubkey) -> Result<()> {
        init_user_state::handler(ctx, referrer)
    }

    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        unstake::handler(ctx)
    }
}
