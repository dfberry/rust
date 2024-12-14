-- Your SQL goes here
CREATE TABLE logfiles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    logfile JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    org_repo TEXT NOT NULL
);