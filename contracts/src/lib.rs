use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod microfund {
    use super::*;

    pub fn initialize_loan(ctx: Context<InitializeLoan>, amount: u64, description: String) -> Result<()> {
        let loan = &mut ctx.accounts.loan;
        loan.borrower = *ctx.accounts.borrower.key;
        loan.amount = amount;
        loan.description = description;
        loan.repaid = false;
        loan.created_at = Clock::get()?.unix_timestamp;
        
        msg!("Loan initialized for amount: {}", amount);
        Ok(())
    }

    pub fn repay_loan(ctx: Context<RepayLoan>) -> Result<()> {
        let loan = &mut ctx.accounts.loan;
        require!(!loan.repaid, LoanError::AlreadyRepaid);
        
        loan.repaid = true;
        loan.repaid_at = Clock::get()?.unix_timestamp;
        
        msg!("Loan repaid successfully");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeLoan<'info> {
    #[account(init, payer = borrower, space = 8 + 32 + 8 + 200 + 1 + 8 + 8)]
    pub loan: Account<'info, LoanAccount>,
    #[account(mut)]
    pub borrower: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RepayLoan<'info> {
    #[account(mut, has_one = borrower)]
    pub loan: Account<'info, LoanAccount>,
    pub borrower: Signer<'info>,
}

#[account]
pub struct LoanAccount {
    pub borrower: Pubkey,
    pub amount: u64,
    pub description: String,
    pub repaid: bool,
    pub created_at: i64,
    pub repaid_at: i64,
}

#[error_code]
pub enum LoanError {
    #[msg("This loan has already been repaid.")]
    AlreadyRepaid,
}
