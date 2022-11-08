use anchor_lang::prelude::*;
use std::mem::size_of;

use crate::{constants::*, error::*, states::*};

#[derive(Accounts)]
pub struct AddBlacklist<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
      seeds = [SETTINGS_SEED],
      bump,
      has_one = admin
    )]
    pub settings: Box<Account<'info, Settings>>,
    
    #[account(
        mut,
        seeds = [BLACKLIST_SEED],
        bump
    )]
    pub blacklist: Box<Account<'info, Blacklist>>
}

pub fn handler(ctx: Context<AddBlacklist>, 
    black_address: Pubkey,
) -> Result<()> {
    let accts = ctx.accounts;
    
    require!(black_address.ne(&Pubkey::default()), CustomError::ZeroAddressDetected);

    accts.blacklist.addresses.push(black_address);
    Ok(())
}
