/*
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
*/
CREATE TABLE MyUser (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

/* 
DROP TABLE MyUser;
*/