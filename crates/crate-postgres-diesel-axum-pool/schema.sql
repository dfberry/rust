-- Enable the pgcrypto extension if it's not already enabled
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE test_table_2 (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    date_created TIMESTAMPTZ NOT NULL DEFAULT NOW()
);