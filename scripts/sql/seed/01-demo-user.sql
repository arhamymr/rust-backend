-- Seed data for development
-- This script will be executed after migrations are complete

-- Insert sample user
INSERT INTO user (id, name, email, email_verified, created_at, updated_at)
VALUES (
    gen_random_uuid(),
    'Demo User',
    'demo@example.com',
    true,
    NOW(),
    NOW()
) ON CONFLICT (id) DO NOTHING;

-- You can add more seed data as needed