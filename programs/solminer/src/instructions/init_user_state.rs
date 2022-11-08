use crate::{constants::*, error::*, states::*};
use anchor_lang::prelude::*;

use std::mem::size_of;

#[derive(Accounts)]
#[instruction(user_key: Pubkey)]
pub struct InitUserState<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [SETTINGS_SEED],
        bump = settings.bump,
    )]
    pub settings: Box<Account<'info, Settings>>,
  
    #[account(
        init,
        payer = payer,
        space = 8 + size_of::<UserState>(),
        seeds = [STATE_SEED, user_key.as_ref()],
        bump
    )]
    pub user_state: Account<'info, UserState>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> InitUserState<'info> {
    fn validate(&self) -> Result<()> {
        Ok(())
    }
}

#[access_control(ctx.accounts.validate())]
pub fn handler(ctx: Context<InitUserState>, user_key: Pubkey) -> Result<()> {
    require!(user_key.ne(&Pubkey::default()), CustomError::ZeroAddressDetected);
    let current_time = Clock::get()?.unix_timestamp as u64;

    let accts = ctx.accounts;
    accts.user_state.user = user_key;
    accts.user_state.bump = *ctx.bumps.get("user_state").unwrap();
    
    accts.settings.members += 1;
    Ok(())
}
