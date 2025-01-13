# Central User Space - Complete Project Overview

## Technology Stack (2024)

```rust
Core:
- Rust 1.75+
- Axum 0.7 (web framework)
- Tokio 1.0 (async runtime)
- PostgreSQL 16 with SQLx 0.7
- Redis 7

Auth & Security:
- jsonwebtoken 9.0
- argon2 0.5
- ring (crypto)
- tower-http (middleware)

Validation & Serialization:
- validator 0.16
- serde 1.0
- serde_json 1.0

Development & Testing:
- sqlx-cli (migrations)
- tracing (logging)
- mockall (mocking)
- tokio-test
- testcontainers-rs
```

## Project Structure

```rust
.
├── Cargo.toml
├── Dockerfile
├── docker-compose.yml
├── migrations/
│   ├── 20240101000000_init.sql
│   └── ...
└── src/
    ├── main.rs
    ├── api/                    # API Layer
    │   ├── middleware/        
    │   │   ├── auth.rs
    │   │   ├── rate_limit.rs
    │   │   └── logging.rs
    │   ├── routes/
    │   │   ├── auth.rs
    │   │   ├── users.rs
    │   │   └── admin.rs
    │   └── handlers/
    │       ├── auth.rs
    │       ├── users.rs
    │       └── admin.rs
    ├── core/                   # Business Logic
    │   ├── auth/
    │   │   ├── service.rs
    │   │   ├── models.rs
    │   │   └── errors.rs
    │   ├── users/
    │   │   ├── service.rs
    │   │   ├── models.rs
    │   │   └── validation.rs
    │   └── security/
    │       ├── encryption.rs
    │       ├── password.rs
    │       └── tokens.rs
    ├── db/                     # Data Layer
    │   ├── migrations/
    │   ├── repositories/
    │   │   ├── users.rs
    │   │   └── audit.rs
    │   └── models/
    │       ├── user.rs
    │       └── audit.rs
    ├── config/
    │   ├── mod.rs
    │   └── settings.rs
    └── utils/
        ├── errors.rs
        ├── logging.rs
        └── validation.rs
```

## Core Features & Implementation

### Authentication System

```md
JWT Configuration:
- Short lived access tokens (15min)
- Longer lived refresh tokens (7 days)
- Rotation on every use
- JWK for signature verification

Password Security:
- Argon2id with modern parameters
- Breached password checking
- Complex password requirements
- Rate limiting on attempts
```

### User Data Model

```sql
-- Core User Table
CREATE TABLE users (
    id UUID PRIMARY KEY,
    email VARCHAR(255) UNIQUE,
    password_hash VARCHAR(255),
    status user_status,
    created_at TIMESTAMPTZ,
    updated_at TIMESTAMPTZ,
    
    -- Optional but recommended
    last_login TIMESTAMPTZ,
    failed_attempts INT,
    locked_until TIMESTAMPTZ
);

-- Extended Profile (Optional)
CREATE TABLE user_profiles (
    user_id UUID PRIMARY KEY,
    -- extensible profile fields
);

-- Security Settings
CREATE TABLE user_security (
    user_id UUID PRIMARY KEY,
    mfa_enabled BOOLEAN,
    mfa_secret TEXT,
    backup_codes JSONB,
    security_keys JSONB
);
```

### API Design (REST)

```rust
Base Path: /api/v1

Authentication:
POST    /auth/register
POST    /auth/login
POST    /auth/refresh
DELETE  /auth/logout
POST    /auth/password/reset
POST    /auth/mfa/enable

Users:
GET     /users/me
PATCH   /users/me
GET     /users/me/sessions
DELETE  /users/me/sessions/{id}

Security:
POST    /security/mfa/verify
POST    /security/mfa/backup
PATCH   /security/password
GET     /security/audit-log
```

### Security Implementation

#### Request Pipeline

1. TLS Termination
2. Rate Limiting Check
3. API Key Validation
4. JWT Validation
5. Permission Check
6. Input Validation
7. Handler Execution
8. Response Sanitization
9. Audit Logging

#### Logging Strategy

```rust
// Structured Logging Fields
{
    "timestamp": "",
    "trace_id": "",
    "user_id": "",
    "ip": "",
    "action": "",
    "status": "",
    "duration_ms": 0,
    "metadata": {}
}
```

### Error Handling

```rust
// Unified Error Response
{
    "error": {
        "code": "string",
        "message": "string",
        "details": {} | null,
        "trace_id": "string"
    }
}
```

## Development Workflow

### Local Development

```bash
# Start infrastructure
docker-compose up -d

# Run migrations
sqlx migrate run

# Start development server
cargo watch -x run

# Run tests
cargo test
```

### Database Migrations

```bash
# Create new migration
sqlx migrate add -r create_users

# Run migrations
sqlx migrate run

# Revert last migration
sqlx migrate revert
```

### Testing Strategy

1. Unit Tests per Module
2. Integration Tests for APIs
3. Property-Based Testing
4. Security Testing
5. Performance Benchmarks

## Production Considerations

### Deployment

- Docker containerization
- Health checks
- Graceful shutdown
- Resource limits
- Monitoring hooks

### Scaling Strategy

- Stateless design
- Redis for session store
- Database connection pooling
- Rate limiting per instance
- Load balancer ready

### Monitoring

- Request metrics
- Error rates
- Response times
- Resource usage
- Security events
