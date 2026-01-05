-- Migration to add Reputation System
ALTER TABLE users ADD COLUMN IF NOT EXISTS reputation_score INTEGER DEFAULT 100;

-- Add a column to track repayment history for more complex scoring later
ALTER TABLE loans ADD COLUMN IF NOT EXISTS due_date TIMESTAMPTZ DEFAULT (NOW() + INTERVAL '30 days');
