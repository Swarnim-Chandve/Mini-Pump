use anchor_lang::prelude::*;

declare_id!("32gPK5Tc9pGKX7bvG5TNqdrdgsZwYaAQ7XGkqB4StkFi");

#[program]
pub mod pumpfun {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
