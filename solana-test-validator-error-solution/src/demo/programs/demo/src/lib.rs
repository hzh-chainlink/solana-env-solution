use anchor_lang::prelude::*;

declare_id!("FsBzY1Qp6PUdbef3bN6rqL1JVQy5Ss8zZqUxPkgNq32K");

#[program]
pub mod demo {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
