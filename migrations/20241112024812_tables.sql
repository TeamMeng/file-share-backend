-- Add migration script here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Users table
CREATE table users (
    id UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
    name VARCHAR(100) NOT NULL,
    email VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL, -- Store hashed password
    public_key TEXT,                -- Store the user's public key
    create_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- File table
CREATE TABLE files (
    id UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE, -- Foreign key to users table
    file_name VARCHAR(255) NOT NULL,
    file_size BIGINT NOT NULL,
    encrypted_aes_key BYTEA NOT NULL, -- Store encrypted AES key
    encrypted_file BYTEA NOT NULL,    -- Store the actual encrypted file content
    iv BYTEA NOT NULL,                -- Initialization vector for AES encryption
    create_at TIMESTAMPTZ DEFAULT NOW()
);

-- Shared links table (with required password and expiration date)
CREATE TABLE shared_links (
    id UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
    file_id UUID REFERENCES files(id) ON DELETE CASCADE,            -- Foreign key to files table
    recipient_user_id UUID REFERENCES users(id) ON DELETE CASCADE,  -- Foreign key to users table
    password VARCHAR(255) NOT NULL,         -- Password protection
    expiration_date TIMESTAMPTZ NOT NULL,   -- Expiration date
    create_at TIMESTAMPTZ DEFAULT NOW()
);
