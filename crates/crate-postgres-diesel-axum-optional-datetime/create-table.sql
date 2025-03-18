CREATE TABLE test_table_optional_datetime (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    success BOOLEAN NOT NULL DEFAULT FALSE,
    optional_date_created timestamp,
    date_created timestamp NOT NULL DEFAULT now() NOT NULL
);