use anchor_lang::prelude::*;

// This is a unique identifier for the MicroFund smart contract on the Solana blockchain.
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod microfund {
    use super::*;

    /// Initializes a new microloan on the blockchain.
    /// This provides a transparent, immutable record of the debt obligation.
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

    /// Records the repayment of a microloan.
    /// Once marked as repaid, the status is permanently updated on-chain.
    pub fn repay_loan(ctx: Context<RepayLoan>) -> Result<()> {
        let loan = &mut ctx.accounts.loan;
        // Ensure the loan hasn't been repaid already to prevent double-repayment logic errors.
        require!(!loan.repaid, LoanError::AlreadyRepaid);
        
        loan.repaid = true;
        loan.repaid_at = Clock::get()?.unix_timestamp;
        
        msg!("Loan repaid successfully");
        Ok(())
    }
}

/// Accounts required for the 'initialize_loan' instruction.
#[derive(Accounts)]
pub struct InitializeLoan<'info> {
    // We initialize a new account for each loan. 
    // Space is calculated based on the fields in the LoanAccount struct.
    #[account(init, payer = borrower, space = 8 + 32 + 8 + 200 + 1 + 8 + 8)]
    pub loan: Account<'info, LoanAccount>,
    #[account(mut)]
    pub borrower: Signer<'info>,
    pub system_program: Program<'info, System>,
}

/// Accounts required for the 'repay_loan' instruction.
#[derive(Accounts)]
pub struct RepayLoan<'info> {
    // Only the original borrower can authorize the repayment record.
    #[account(mut, has_one = borrower)]
    pub loan: Account<'info, LoanAccount>,
    pub borrower: Signer<'info>,
}

/// Data structure for storing loan information on-chain.
#[account]
pub struct LoanAccount {
    pub borrower: Pubkey,    // Public key of the user who took the loan
    pub amount: u64,          // Amount borrowed in lamports (or simulated units)
    pub description: String,  // Metadata about the loan's purpose
    pub repaid: bool,         // Repayment status
    pub created_at: i64,      // Timestamp of loan creation
    pub repaid_at: i64,       // Timestamp of loan repayment
}

#[error_code]
pub enum LoanError {
    #[msg("This loan has already been repaid.")]
    AlreadyRepaid,
}
