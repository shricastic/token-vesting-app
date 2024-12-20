#![allow(clippy::result_large_err)]

use std::alloc::System;

use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::TokenInterface, token::{Mint, TokenAccount}};

declare_id!("coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF");

#[program]
pub mod tokenvestingapp {
    use std::i64;

    use anchor_spl::{token::TransferChecked, token_interface};

    use super::*;
    
    pub fn create_vesting_account(ctx: Context<CreateVestingAccount>, company_name: String) -> Result<()> {
        *ctx.accounts.vesting_account = VestingAccount{
            owner: ctx.accounts.signer.key(),
            mint: ctx.accounts.mint.key(),
            treasury_token_account: ctx.accounts.treasury_token_account.key(),
            company_name,
            treasury_bump: ctx.bumps.treasury_token_account,
            bump: ctx.bumps.vesting_account,
        };

        Ok(())
    }

    pub fn create_employee_account(
        ctx: Context<CreateEmployeeAccount>,
        start_time: i64,
        end_time: i64,
        total_amount: u64,
        cliff_time: i64
        ) -> Result<()> {
        *ctx.accounts.employee_account = EmployeeAccount {
            beneficiery: ctx.accounts.beneficiery.key(),
            start_time,
            end_time,
            total_amount,
            total_withdrawn: 0,
            cliff_time,
            vesting_account: ctx.accounts.vesting_account.key(),
            bump: ctx.accounts.vesting_account.bump
        };

        Ok(())
    }

    pub fn claim_token(
        ctx: Context<CreateClaimTokens>,
        company_name: String
    ) -> Result<()> {
        let employee_account = &mut ctx.accounts.employee_account;
        
        let now = Clock::get()?.unix_timestamp;

        if(now<employee_account.cliff_time){
            return Err(ErrorCode:: ClaimNotAvailableYet.into());
        }
        
        let time_since_start = now.saturating_sum(employee_account.start_time);
        let total_vesting_time = employee_account.end_time.saturating_sub(employee_account.start_time);

        if total_vesting_time == 0 {
            return Err(ErrorCode::InvalidVestingPeriod.into());
        }

        let vested_amount = if now >= employee_account.end_time{
            employee_account.total_amount
        } else {
            match employee_account.total_amount.checked_mul(time_since_start as u64){
                Some(product) => {
                    product / total_vesting_time as u64
                },
                None => {
                    return Err(ErrorCode::CalculationOverflow.into())
                }
            }
        };

        let claimable_amount = vested_amount.saturating_sub(employee_account.total_withdrawn);

        if claimable_amount == 0 {
            return Err(ErrorCode::NothingToClaim.into());
        }

        let transfer_cpi_account = TransferChecked{
            from: ctx.accounts.treasury_token_account.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.employee_account.to_account_info(),
            authority: ctx.accounts.treasury_token_account.to_account_info(),
        };
        
        let cpi_program = ctx.accounts.token_program.to_account_info();

        let signer_seeds: &[&[&[u8]]] = &[
            &[b"vesting_treasury",
                ctx.accounts.vesting_account.company_name.as_ref(),
                &[ctx.accounts.vesting_account.treasury_bump]
            ]
        ];

        let cpi_context = CpiContext::new(cpi_program, transfer_cpi_accounts).with_signer(signer_seeds);

        let decimals = ctx.accounts.mint.decimal;

        token_interface::transfer_checked(cpi_context, claimable_amount as u64, decimals)?;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(company_name: String)]
pub struct CreateVestingAccount<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        space = VestingAccount::INIT_SPACE + 8,
        payer = signer,
        seeds = [company_name.as_ref()],
        bump,
    )]
    pub vesting_account: Account<'info, VestingAccount>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        token::mint = mint,
        token::authority = treasury_token_account,
        payer = signer,
        seeds = [b"vesting_treasury", company_name.as_bytes()],
        bump
    )]
    pub treasury_token_account: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>

}

#[derive(Accounts)]
pub struct CreateEmployeeAccount<'info>{
    #[account(mut)]
    pub owner: Signer<'info>,
    pub beneficiery: SystemAccount<'info>,
    
    #[account(has_one = owner)]
    pub vesting_account: Account<'info, VestingAccount>,

    #[account(
        init, 
        space = 8 + EmployeeAccount::INIT_SPACE,
        payer = owner,
        seeds = [b"employee_vesting", beneficiery.key().as_ref(), vesting_account.key().as_ref()],
        bump
    )]
    pub employee_account: Account<'info, EmployeeAccount>,
    
    pub system_program: Program<'info, System>,

}

#[derive(Accounts)]
#[instruction(company_name:String)]
pub struct CreateClaimTokens<'info> {
    #[account(mut)]
    pub beneficiery: Signer<'info>,

    #[account(
        mut,
        seeds = [b"employee_vesting", beneficiery.key().as_ref(), vesting_account.key().as_ref()],
        bump = employee_account.bump,
        has_one = beneficiery,
        has_one = vesting_account,
    )]
    pub employee_account: Account<'info, EmployeeAccount>,

    #[account(
        mut,
        seeds = [company_name.as_ref()],
        bump = vesting_account.bump,
        has_one = treasury_token_account,
        has_one = mint
    )]
    pub vesting_account:Account<'info, VestingAccount>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub treasury_token_account: InterfaceAccount<'info, TokenAccount>,
    
    #[account(
        init_if_needed,
        payer = beneficiery,
        associated_token::mint = mint,
        associated_token::authority = treasury_token_account,
        associated_token::token_program = token_program
    )]
    pub employee_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub system_program: Program<'info, System>
}

#[account]
#[derive(InitSpace)]
pub struct VestingAccount{
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub treasury_token_account: Pubkey,
    #[max_len(50)]
    pub company_name: String,
    pub treasury_bump: u8,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct EmployeeAccount{
    pub beneficiery: Pubkey,
    pub start_time: i64,
    pub end_time: i64,
    pub cliff_time: i64,
    pub vesting_account: Pubkey,
    pub total_amount: u64,
    pub total_withdrawn: u64,
    pub bump: u8
}

#[error_code]
pub enum ErrorCode{
    #[msg("Claim not available yet")]
    ClaimNotAvailableYet,

    #[msg("Invalid veting period")]
    InvalidVestingPeriod,


    #[msg("Calculation overflow")]
    CalculationOverflow,

    #[msg("Nothing to claim")]
    NothingToClaim
}
