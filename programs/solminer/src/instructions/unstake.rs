use crate::{constants::*, error::*, states::*};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    program::{invoke, invoke_signed},
    system_instruction,
};
use std::mem::size_of;

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
      seeds = [SETTINGS_SEED],
      bump = settings.bump,
      has_one = dev_wallet,
      has_one = pool
    )]
    pub settings: Box<Account<'info, Settings>>,

    #[account(
      mut,
      seeds = [BLACKLIST_SEED],
      bump
    )]
    pub blacklist: Box<Account<'info, Blacklist>>,

    #[account(mut)]
    /// CHECK: checked with state variable
    pub dev_wallet: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: pool account is pda for storing sols
    pub pool: AccountInfo<'info>,

    #[account(
      mut,
      seeds = [STATE_SEED, user.key().as_ref()],
      bump = user_state.bump
    )]
    pub user_state: Account<'info, UserState>,

    #[account(
      mut,
      seeds = [DATA_SEED, user.key().as_ref(), invest_data.seed_key.as_ref()],
      bump = invest_data.bump,
      close = pool
    )]
    pub invest_data: Account<'info, InvestData>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> Unstake<'info> {
    fn validate(&self) -> Result<()> {
        Ok(())
    }
}

#[access_control(ctx.accounts.validate())]
pub fn handler(ctx: Context<Unstake>) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp as u64;
   
    let accts = ctx.accounts;
    if accts.blacklist.addresses.contains(&accts.user.key()) {
      return Err(CustomError::Blocked.into());
    }

    let mut withdraw_tax = 0;
    if accts.invest_data.days < WITHDRAW_LIMIT {
        withdraw_tax = accts.invest_data.active_balance
            .checked_mul(accts.settings.withdraw_tax_rate)
            .unwrap()
            .checked_div(FEE_DIVIDER)
            .unwrap();
    }

    let mut withdraw_without_tax = accts.invest_data.active_balance
        .checked_sub(withdraw_tax)
        .unwrap();

    if withdraw_without_tax > accts.pool.lamports() {
      withdraw_without_tax = accts.pool.lamports();
    }
    let withdraw_fee = accts.invest_data.active_balance
        .checked_mul(accts.settings.fee)
        .unwrap()
        .checked_div(FEE_DIVIDER)
        .unwrap();
    let real_withdraw = withdraw_without_tax.checked_sub(withdraw_fee).unwrap();

    let signer_seeds: &[&[&[u8]]] = &[&[POOL_SEED.as_ref(), &[accts.settings.pool_bump]]];

    invoke_signed(
        &system_instruction::transfer(&accts.pool.key(), &accts.user.key(), real_withdraw),
        &[
            accts.pool.to_account_info(),
            accts.user.to_account_info(),
            accts.system_program.to_account_info(),
        ],
        signer_seeds,
    )?;
    invoke_signed(
        &system_instruction::transfer(&accts.pool.key(), &accts.dev_wallet.key(), withdraw_fee),
        &[
            accts.pool.to_account_info(),
            accts.dev_wallet.to_account_info(),
            accts.system_program.to_account_info(),
        ],
        signer_seeds,
    )?;

    accts.user_state.matrix_count = accts.user_state.matrix_count - 1;
    Ok(())
}
