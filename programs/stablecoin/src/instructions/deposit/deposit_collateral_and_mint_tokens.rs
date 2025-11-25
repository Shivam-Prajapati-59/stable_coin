use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;

use crate::{constants::SEED_CONFIG_ACCOUNT, state::Collateral, Config, SEED_COLLATERAL_ACCOUNT};

#[derive(Accounts)]
pub struct DepositCollateralAndMintTokens<'info> {
    #[account(mut)]
    pub depositer: Signer<'info>,

    #[account(
        seeds = [SEED_CONFIG_ACCOUNT],
        bump,
        has_one = mint_account,
    )]
    pub config_account: Account<'info, Config>,

    #[account(mut)]
    pub mint_account: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = depositer,
        space = 8 + Collateral::INIT_SPACE,
        seeds = [SEED_COLLATERAL_ACCOUNT, depositer.key().as_ref()],
        bump,
    )]
    pub collateral_account: Account<'info, Collateral>,

    #[account(
        mut,
        seeds = [SEED_SOL_Account]
    )]
    pub system_program: Program<'info, System>,
}
