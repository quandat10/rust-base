-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";


CREATE TABLE
  "accounts" (
    id serial PRIMARY KEY,
    code UUID UNIQUE NOT NULL DEFAULT (uuid_generate_v4()),
    name VARCHAR(100) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    phone VARCHAR(255) UNIQUE,
    verified BOOLEAN NOT NULL DEFAULT FALSE,
    password VARCHAR(100) NOT NULL,
    role VARCHAR(50) NOT NULL DEFAULT 'user',
    -- user, admin
    created_at TIMESTAMP
    WITH
      TIME ZONE DEFAULT NOW(),
      updated_at TIMESTAMP
    WITH
      TIME ZONE DEFAULT NOW()
  );


CREATE INDEX accounts_email_idx ON accounts (email);
