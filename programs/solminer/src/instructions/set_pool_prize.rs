use anchor_lang::prelude::*;
use std::mem::size_of;

use crate::{constants::*, error::*, states::*};

#[derive(Accounts)]
pub struct SetPoolPrize<'info> {
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

pub fn handler(ctx: Context<SetPoolPrize>,
    limit_minutes: u64,
    ratio: u64,
) -> Result<()> {
    let accts = ctx.accounts;
    accts.settings.pool_prize_limit = limit_minutes * 60;
    accts.settings.pool_prize_ratio = ratio;
    Ok(())
}