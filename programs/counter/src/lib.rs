use anchor_lang::prelude::*;

declare_id!("6rgzFaFsP74g72s3ULV9w2hFqh7rCMuaDZ8yy5HiKMZm");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
