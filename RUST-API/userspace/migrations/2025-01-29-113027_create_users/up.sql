-- Enable UUID extension if not already enabled
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Users Table (Authentication & Core User Data)
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash TEXT,  -- Nullable for SSO users
    auth_type VARCHAR(10) NOT NULL CHECK (auth_type IN ('password', 'sso')),
    sso_provider VARCHAR(50),  -- Nullable for SSO users
    user_status VARCHAR(10) DEFAULT 'active' CHECK (user_status IN ('active', 'banned', 'suspended')),
    theme_preference VARCHAR(10) DEFAULT 'dark' CHECK (theme_preference IN ('dark', 'light'))
);

-- User Profiles Table
CREATE TABLE user_profiles (
    id SERIAL PRIMARY KEY,
    user_id UUID NOT NULL UNIQUE REFERENCES users(id) ON DELETE CASCADE,
    display_name VARCHAR(100),
    profile_picture TEXT,
    bio TEXT
);

-- User Socials Table
CREATE TABLE user_socials (
    id SERIAL PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    platform VARCHAR(50) NOT NULL, 
    url TEXT NOT NULL,
    UNIQUE(user_id, platform)
);

-- User Stats Table
CREATE TABLE user_stats (
    user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMP DEFAULT NOW(),
    last_login TIMESTAMP,
    last_activity TIMESTAMP,
    last_login_ip INET
);

-- Table for Tracking Additional IP Addresses
CREATE TABLE user_ips (
    id SERIAL PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    ip_address INET NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(user_id, ip_address)
);
