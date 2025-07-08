#![allow(unexpected_cfs)]
#![allow(deprecated)]

use anchor_lang::{accounts::system_account, prelude::*, solana_program::lamports};

use crate::instruction::Initialize;

declare_id!("GrCVa5RFaTqHnq5xvMaZVicDXfuu2Xrp2tmFZDLFcUsM");

#[program]
pub mod anchor_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize <info> {
    #[account(
        mut
    )]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        space = Vaultstate::INIT_SPACE,
        seeds = [b"state", user.key().as_ref()],
        bump
    )]
    #[account(
        mut,
        seeds = [b"vault", user.key().as_ref()],
        bump
    )]
    pub vault_state: Account<'info, Vaultstate>,

    pub vault: system_account<'info>,
    pub system_program: Program<'info, System>,

}
#[derive(accounts)]

pub struct Deposit<'info> {
    #[account(
        mut,
        seeds = [b"vault", user.key().as_ref()],
        bump
    )]

}
impl
    pub vault: Account<'info, Vaultstate>,
    
impl <'info> Initialize {
    pub fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()> {
        let rent_exempt: u64 = Rent::get()?.minimum_balance(self.vault.to_account_info().data_len());
        let cpi_program: AccountInfo<'_> = self.system_program.to_account_info();
        let cpi_accounts: Transfer<'_> = Transfer {
            from: self.user.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let cpi_ctx: CpiContext<'_, '_, '_, '_, _> =
            CpiContext::new(cpi_program, cpi_accounts);
            transfer(cpi_ctx, lamports: rent_exempt)?;
        Ok(())
    }
}
#[account]
pub struct Vaultstate {
    pub vault_bump: u8,
    pub vault_bump: u8,
}

impl Space for Vaultstate {
    const INIT_SPACE: usize = 8 + 1 + 1; // 8 bytes for discriminator + 1 byte for vault_bump + 1 byte for vault_bump
}   