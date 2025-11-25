use anchor_lang::prelude::*;

use constants::*;
use instructions::*;
use state::*;
mod constants;
mod instructions;
mod state;

declare_id!("9oGaNNvJSeetJsA24oWva7s2Ma6TDExngGhNtGDgXBqX");

#[program]
pub mod stablecoin {

    use super::*;

    pub fn initialize_config(ctx: Context<InitializeConfig>) -> Result<()> {
        process_initialize_config(ctx)
    }

    pub fn update_config(ctx: Context<UpdateConfig>, min_health_factor: u64) -> Result<()> {
        process_update_config(ctx, min_health_factor)
    }
}
