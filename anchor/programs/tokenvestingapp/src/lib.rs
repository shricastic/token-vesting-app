#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF");

#[program]
pub mod tokenvestingapp {
    use super::*;

  pub fn close(_ctx: Context<CloseTokenvestingapp>) -> Result<()> {
    Ok(())
  }

  pub fn decrement(ctx: Context<Update>) -> Result<()> {
    ctx.accounts.tokenvestingapp.count = ctx.accounts.tokenvestingapp.count.checked_sub(1).unwrap();
    Ok(())
  }

  pub fn increment(ctx: Context<Update>) -> Result<()> {
    ctx.accounts.tokenvestingapp.count = ctx.accounts.tokenvestingapp.count.checked_add(1).unwrap();
    Ok(())
  }

  pub fn initialize(_ctx: Context<InitializeTokenvestingapp>) -> Result<()> {
    Ok(())
  }

  pub fn set(ctx: Context<Update>, value: u8) -> Result<()> {
    ctx.accounts.tokenvestingapp.count = value.clone();
    Ok(())
  }
}

#[derive(Accounts)]
pub struct InitializeTokenvestingapp<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,

  #[account(
  init,
  space = 8 + Tokenvestingapp::INIT_SPACE,
  payer = payer
  )]
  pub tokenvestingapp: Account<'info, Tokenvestingapp>,
  pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct CloseTokenvestingapp<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,

  #[account(
  mut,
  close = payer, // close account and return lamports to payer
  )]
  pub tokenvestingapp: Account<'info, Tokenvestingapp>,
}

#[derive(Accounts)]
pub struct Update<'info> {
  #[account(mut)]
  pub tokenvestingapp: Account<'info, Tokenvestingapp>,
}

#[account]
#[derive(InitSpace)]
pub struct Tokenvestingapp {
  count: u8,
}
