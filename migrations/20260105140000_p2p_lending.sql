-- Migration to support P2P Lending
ALTER TABLE loans ADD COLUMN IF NOT EXISTS lender_id UUID REFERENCES users(id);
ALTER TABLE loans ALTER COLUMN status SET DEFAULT 'pending';

-- Add a marketplace view for pending loans
CREATE OR REPLACE VIEW marketplace AS
SELECT l.*, u.username as borrower_username
FROM loans l
JOIN users u ON l.user_id = u.id
WHERE l.status = 'pending';
