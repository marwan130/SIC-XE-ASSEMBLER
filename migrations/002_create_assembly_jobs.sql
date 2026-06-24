-- Create assembly_jobs table
CREATE TABLE IF NOT EXISTS assembly_jobs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    title TEXT NOT NULL DEFAULT 'Untitled',
    code TEXT NOT NULL,
    intermediate TEXT,
    pass1 TEXT,
    symb_table TEXT,
    lit_table TEXT,
    object_program TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create index on user_id for faster user-specific queries
CREATE INDEX IF NOT EXISTS idx_assembly_jobs_user_id ON assembly_jobs(user_id);
-- Create index on created_at for chronological ordering
CREATE INDEX IF NOT EXISTS idx_assembly_jobs_created_at ON assembly_jobs(created_at DESC);
