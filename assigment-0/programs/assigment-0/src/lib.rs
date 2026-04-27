use anchor_lang::prelude::*;

declare_id!("Brx3Qx8eyGPZyrn2bDWLkCSKC58sDZg5xA1mKcJgd3zQ");

#[program]
pub mod assigment_0 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
