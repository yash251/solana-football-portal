use anchor_lang::prelude::*;

declare_id!("Gri4oQVgsq4k2r2GDgMVjnnzswKKCvpFXY91Q9xywg5c"); // pub key

#[program]
pub mod solana_football_portal {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
