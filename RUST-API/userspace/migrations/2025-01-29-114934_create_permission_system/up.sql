-- Your SQL goes here
-- Roles Table (Defines Available Roles)
CREATE TABLE roles (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) UNIQUE NOT NULL,
    description TEXT
);

-- User Roles Table (Assigns Multiple Roles to a User)
CREATE TABLE user_roles (
    id SERIAL PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role_id INT NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    UNIQUE(user_id, role_id)
);

-- Permissions Table (Defines Available Permissions)
CREATE TABLE permissions (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) UNIQUE NOT NULL,
    description TEXT
);

-- Role Permissions Table (Assigns Permissions to Roles)
CREATE TABLE role_permissions (
    id SERIAL PRIMARY KEY,
    role_id INT NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    permission_id INT NOT NULL REFERENCES permissions(id) ON DELETE CASCADE,
    UNIQUE(role_id, permission_id)
);

-- User Permissions Table (Explicit Overrides)
CREATE TABLE user_permissions (
    id SERIAL PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    permission_id INT NOT NULL REFERENCES permissions(id) ON DELETE CASCADE,
    value BOOLEAN, -- TRUE = Explicit Allow, FALSE = Explicit Deny, NULL = Follow Role
    UNIQUE(user_id, permission_id) -- No duplicate user-permission entries
);
