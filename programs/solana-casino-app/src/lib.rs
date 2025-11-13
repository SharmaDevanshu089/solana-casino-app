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

pub fn place_bet(ctx: Context<PlaceBet>, amount: u64) -> Result<()> {
    // validation
    require!(amount > 0, CasinoError::InvalidBet);

    // transfer lamports from player -> vault (both must be mutable in accounts)
    let payer_info = &mut ctx.accounts.payer.to_account_info();
    let vault_info = &mut ctx.accounts.vault.to_account_info();

    // subtract then add (checked arithmetic)
    **payer_info.try_borrow_mut_lamports()? = payer_info
        .lamports()
        .checked_sub(amount)
        .ok_or(CasinoError::InvalidBet.into())?;
    **vault_info.try_borrow_mut_lamports()? = vault_info
        .lamports()
        .checked_add(amount)
        .ok_or(CasinoError::MathOverflow.into())?;

    // update vault state
    let vault_account = &mut ctx.accounts.vault;
    vault_account.total_earnings = vault_account
        .total_earnings
        .checked_add(amount)
        .ok_or(CasinoError::MathOverflow)?;

    msg!("Bet received: {} lamports", amount);
    Ok(())
}
