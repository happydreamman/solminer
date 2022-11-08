use anchor_lang::prelude::*;
use std::mem::size_of;

use crate::{constants::*, error::*, states::*};
use std::str::FromStr;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        seeds = [SETTINGS_SEED],
        bump,
        space = 8 + size_of::<Settings>()
    )]
    pub settings: Box<Account<'info, Settings>>,

    #[account(
        seeds = [POOL_SEED],
        bump
    )]
    /// CHECK: pool account is pda for storing sols
    pub pool: AccountInfo<'info>,

    /// CHECK: no need to check
    pub dev_wallet: AccountInfo<'info>,
    /// CHECK: no need to check
    pub marketing_wallet: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> Initialize<'info> {
    pub fn validate(&self) -> Result<()> {
        Ok(())
    }
}

/// Initialize Staking Program for the first time
/// to init global state with some data for validation
///
#[access_control(ctx.accounts.validate())]
pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    let accts = ctx.accounts;
    let admin_key = accts.admin.key();
    accts.settings.admin = admin_key;
    accts.settings.pool = accts.pool.key();

    accts.settings.roi = 10115; // 1.0115 x
    accts.settings.fee = 500; // 5%
    accts.settings.ref_fee = 1000; // 10%
    accts.settings.withdraw_tax_rate = 5000; // 50%
    accts.settings.compound_fee = 100; // 1%

    accts.settings.bump = *ctx.bumps.get("settings").unwrap();
    accts.settings.pool_bump = *ctx.bumps.get("pool").unwrap();
    
    accts.settings.dev_wallet = admin_key;
    accts.settings.marketing_wallet = admin_key;
    accts.settings.pool_prize_limit = DAY_IN_SEC;
    accts.settings.pool_prize_ratio = 100; // 1%

    accts.settings.last_deposit_user = accts.admin.key();
    accts.settings.last_deposit_time = Clock::get()?.unix_timestamp as u64;
    Ok(())
}
