-- Migration for Persistent Transaction Ledger
CREATE TABLE IF NOT EXISTS platform_transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    activity_type VARCHAR(100) NOT NULL, -- "LOAN_CREATED", "REPAYMENT", "SAVINGS_DEPOSIT"
    description TEXT NOT NULL,
    amount DECIMAL NOT NULL,
    signature VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
