-- Create users table
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email TEXT UNIQUE,
    password_hash TEXT,
    name TEXT NOT NULL,
    avatar_url TEXT,
    provider TEXT NOT NULL DEFAULT 'local',
    provider_id TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create index on email for faster lookups
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
-- Create index on provider and provider_id for OAuth lookups
CREATE INDEX IF NOT EXISTS idx_users_provider ON users(provider, provider_id);
