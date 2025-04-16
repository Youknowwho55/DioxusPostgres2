-- Add migration script here
-- Correct enum naming (loan_officer instead of loan_officer)
CREATE TYPE user_role AS ENUM ('admin', 'loan_officer', 'processor');

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) NOT NULL UNIQUE CHECK (username ~ '^[a-zA-Z0-9_]{3,50}$'),
    first_name VARCHAR(100) NOT NULL CHECK (first_name <> ''),
    last_name VARCHAR(100) NOT NULL CHECK (last_name <> ''),
    email VARCHAR(255) NOT NULL UNIQUE CHECK (email ~* '^[A-Za-z0-9._%-]+@[A-Za-z0-9.-]+[.][A-Za-z]+$'),
    password_hash TEXT NOT NULL, -- Changed from password to password_hash
    role user_role NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_login TIMESTAMPTZ,
    CONSTRAINT valid_username CHECK (username ~ '^[a-zA-Z0-9_]{3,50}$')
);

-- Indexes for faster lookups
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_is_active ON users(is_active);