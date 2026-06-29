-- Add oauth_token column to users table
ALTER TABLE users ADD COLUMN IF NOT EXISTS oauth_token TEXT;
