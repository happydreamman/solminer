use crate::{constants::*, error::*, states::*};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    native_token::LAMPORTS_PER_SOL,
    program::{invoke, invoke_signed},
    system_instruction,
};
use std::mem::size_of;

#[derive(Accounts)]
#[instruction(amt: u64, seed_key: Pubkey)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
      mut,
      seeds = [SETTINGS_SEED],
      bump = settings.bump,
      has_one = dev_wallet,
      has_one = pool,
      has_one = last_deposit_user
    )]
    pub settings: Box<Account<'info, Settings>>,

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
      init,
      seeds = [DATA_SEED, user.key().as_ref(), seed_key.as_ref()],
      bump,
      space = 8 + size_of::<InvestData>(),
      payer = user
    )]
    pub invest_data: Account<'info, InvestData>,
    
    #[account(mut)]
    /// CHECK: checked with state variable
    pub referrer: AccountInfo<'info>,

    #[account(
      mut,
      seeds = [STATE_SEED, referrer.key().as_ref()],
      bump = ref_user_state.bump
    )]
    pub ref_user_state: Account<'info, UserState>,

    #[account(mut)]
    /// CHECK: checked with state variable
    pub last_deposit_user: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> Deposit<'info> {
    fn validate(&self, amount: u64) -> Result<()> {
        require!(
            amount >= LAMPORTS_PER_SOL && amount <= 1000 * LAMPORTS_PER_SOL,
            CustomError::InvalidAmount
        );
        require!(
            self.referrer.key().ne(&Pubkey::default()),
            CustomError::InvalidReferrer
        );
        require!(
          self.settings.miner_started == 1,
          CustomError::MinerNotStarted
        );
        /*require!(
          self.user_state.matrix_count <= 1,
          CustomError::MatrixCountOverflow
        );*/
        Ok(())
    }
}

#[access_control(ctx.accounts.validate(amount))]
pub fn handler(ctx: Context<Deposit>, amount: u64, seed_key: Pubkey) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp as u64;

    let accts = ctx.accounts;
    
    // check referral
    if accts.user_state.referrer.eq(&Pubkey::default()) {
        accts.user_state.referrer = accts.referrer.key();
    } else {
        require!(accts.user_state.referrer.eq(&accts.referrer.key()), CustomError::InvalidReferrer);
    }
    // ref fee
    let ref_fee = amount
        .checked_mul(accts.settings.ref_fee)
        .unwrap()
        .checked_div(FEE_DIVIDER)
        .unwrap();


    accts.user_state.matrix_count = accts.user_state.matrix_count + 1;

    accts.invest_data.user = accts.user.key();
    accts.invest_data.amount = amount;
    accts.invest_data.invest_time = current_time;
    accts.invest_data.seed_key = seed_key;
    accts.invest_data.bump = *ctx.bumps.get("invest_data").unwrap();
    accts.invest_data.last_roi_time = current_time;

    
    if accts.settings.last_deposit_time + accts.settings.pool_prize_limit < current_time {
        let signer_seeds: &[&[&[u8]]] = &[&[POOL_SEED.as_ref(), &[accts.settings.pool_bump]]];
        let pool_prize = accts.pool.lamports()
            .checked_mul(accts.settings.pool_prize_ratio)
            .unwrap()
            .checked_div(FEE_DIVIDER)
            .unwrap();
        invoke_signed(
            &system_instruction::transfer(&accts.pool.key(), &accts.last_deposit_user.key(), pool_prize),
            &[
                accts.pool.to_account_info(),
                accts.last_deposit_user.to_account_info(),
                accts.system_program.to_account_info(),
            ],
            signer_seeds,
        )?;
        accts.settings.last_pool_winner = accts.last_deposit_user.key();
        accts.settings.last_pool_reward = pool_prize;
    }

    accts.settings.last_deposit_time = current_time;
    accts.settings.last_deposit_user = accts.user.key();
   
    // calc deposit fee and rest amount
    let deposit_fee = amount
        .checked_mul(accts.settings.fee)
        .unwrap()
        .checked_div(FEE_DIVIDER)
        .unwrap();

    let real_amount = amount.checked_sub(deposit_fee).unwrap();
    let mut contract_amount = real_amount;
    // send referral
    if accts.referrer.key().ne(&accts.user.key()) {
      accts.ref_user_state.referred_count = accts.ref_user_state.referred_count + 1;
      accts.ref_user_state.referral_reward = accts.ref_user_state.referral_reward + ref_fee;
      invoke(
        &system_instruction::transfer(&accts.user.key(), &accts.referrer.key(), ref_fee),
        &[
            accts.user.to_account_info(),
            accts.referrer.to_account_info(),
            accts.system_program.to_account_info(),
        ],
      )?;
      contract_amount = real_amount.checked_sub(ref_fee).unwrap();
    }

    accts.invest_data.active_balance = real_amount;
    
    // send deposit_fee to treasury
     invoke(
        &system_instruction::transfer(&accts.user.key(), &accts.dev_wallet.key(), deposit_fee),
        &[
            accts.user.to_account_info(),
            accts.dev_wallet.to_account_info(),
            accts.system_program.to_account_info(),
        ],
    )?;
    // add funds to contract pool
    invoke(
        &system_instruction::transfer(&accts.user.key(), &accts.pool.key(), contract_amount),
        &[
            accts.user.to_account_info(),
            accts.pool.to_account_info(),
            accts.system_program.to_account_info(),
        ],
    )?;

    Ok(())
}
