use anchor_lang::prelude::*;

declare_id!("BHBfQYJGDEq6FksReGGi3govXNQGCgYUMz7v7tnJr8h6");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
