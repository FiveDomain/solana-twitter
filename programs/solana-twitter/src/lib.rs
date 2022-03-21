use anchor_lang::prelude::*;

declare_id!("452VBruZdiZxs6mCZzLgx3R1ebUjCiXtCw67YACG6piM");

#[program]
pub mod solana_twitter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
