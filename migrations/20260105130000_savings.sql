-- Up Migration (Add Savings)
CREATE TABLE IF NOT EXISTS savings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    amount DECIMAL NOT NULL DEFAULT 0,
    goal_name VARCHAR(255),
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS savings_transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    savings_id UUID NOT NULL REFERENCES savings(id),
    amount DECIMAL NOT NULL,
    transaction_type VARCHAR(50) NOT NULL, -- deposit, withdrawal
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
