use anchor_lang::prelude::*;
use std::mem::size_of;

use crate::{constants::*, error::*, states::*};

#[derive(Accounts)]
pub struct StartMiner<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [SETTINGS_SEED],
        bump,
        has_one = admin
    )]
    pub settings: Box<Account<'info, Settings>>,
}

pub fn handler(ctx: Context<StartMiner>) -> Result<()> {
    let accts = ctx.accounts;
    require!(accts.settings.miner_started == 1, CustomError::MinerAlreadyStarted);
    accts.settings.miner_started = 1;
    Ok(())
}