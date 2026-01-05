-- Seed Data for MicroFund Africa Demo

-- Create a demo user (password is 'password123')
-- Hash generated for 'password123' using Argon2 (placeholder)
INSERT INTO users (id, username, email, password_hash) 
VALUES (
    'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 
    'demo_user', 
    'demo@microfund.africa', 
    '$argon2id$v=19$m=4096,t=3,p=1$c2FsdHNhbHQ$m7Lp2D8zF4j9e1Q/qV7X9A'
) ON CONFLICT DO NOTHING;

-- Create some sample loans
INSERT INTO loans (id, user_id, amount, status, description, created_at)
VALUES 
    (gen_random_uuid(), 'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 50.00, 'approved', 'Farm Seeds for Maize', NOW() - INTERVAL '2 days'),
    (gen_random_uuid(), 'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 25.50, 'repaid', 'Mobile Phone Repair', NOW() - INTERVAL '10 days'),
    (gen_random_uuid(), 'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 100.00, 'pending', 'Water Pump Installation', NOW())
ON CONFLICT DO NOTHING;
