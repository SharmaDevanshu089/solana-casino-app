use anchor_lang::prelude::*;

declare_id!("8cEcfm2JA9itFW2gLSmJ91Fq3LPMYmp4V17heZUupPJX");

#[program]
pub mod solana_casino_app {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[error_code]
pub enum CasinoError {
    #[msg("Bet amount must be > 0")]
    InvalidBet,
    #[msg("Overflow")]
    MathOverflow,
}
