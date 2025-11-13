use anchor_lang::prelude::*;

#[account]
pub struct Vault {
    pub bump: u8,
    pub total_earnings: u64,
}

#[error_code]
pub enum CasinoError {
    #[msg("Bet amount must be > 0")]
    InvalidBet,
    #[msg("Overflow")]
    MathOverflow,
}
declare_id!("8cEcfm2JA9itFW2gLSmJ91Fq3LPMYmp4V17heZUupPJX");

#[program]
pub mod solana_casino_app {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
    pub fn place_bet(ctx: Context<PlaceBet>, amount: u64) -> Result<()> {
        require!(amount > 0, CasinoError::InvalidBet);

        let payer_info = &mut ctx.accounts.payer.to_account_info();
        let vault_info = &mut ctx.accounts.vault.to_account_info();

        // subtract from player
        let payer_new = payer_info
            .lamports()
            .checked_sub(amount)
            .ok_or(CasinoError::InvalidBet)?; // <-- FIXED
        **payer_info.try_borrow_mut_lamports()? = payer_new;

        // add to vault
        let vault_new = vault_info
            .lamports()
            .checked_add(amount)
            .ok_or(CasinoError::MathOverflow)?; // <-- FIXED
        **vault_info.try_borrow_mut_lamports()? = vault_new;

        // update vault state
        let vault_account = &mut ctx.accounts.vault;
        vault_account.total_earnings = vault_account
            .total_earnings
            .checked_add(amount)
            .ok_or(CasinoError::MathOverflow)?; // <-- FIXED

        msg!("Bet received: {} lamports", amount);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
#[derive(Accounts)]
pub struct PlaceBet<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut, seeds = [b"vault"], bump)]
    pub vault: Account<'info, Vault>,

    pub system_program: Program<'info, System>,
}
