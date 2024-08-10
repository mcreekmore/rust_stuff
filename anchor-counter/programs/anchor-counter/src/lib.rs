use anchor_lang::prelude::*;

declare_id!("ConRmQj8f3K457fSEaM6PL8CaKUq9SvaCrZxktvNJgzR");

#[program]
pub mod anchor_counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
