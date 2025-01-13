# Simplified Central User Space - Project Overview

## Technology Stack (2024)

```md
Core:
- Rust 1.75+
- Axum 0.7 (web framework)
- Tokio 1.0 (async runtime)
- PostgreSQL 16 with SQLx 0.7

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
```

## Project Structure

```md
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
    │   │   ├── auth.rs        # JWT validation
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
    │   │   └── jwt.rs         # JWT handling
    │   ├── users/
    │   │   ├── service.rs
    │   │   ├── models.rs
    │   │   └── validation.rs
    │   └── security/
    │       ├── encryption.rs
    │       └── password.rs
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
- Access tokens (15min lifetime)
- Refresh tokens (7 days lifetime)
- JWK for signature verification
- jti claim for token identification

Claims Structure:
{
    "sub": "user_id",
    "jti": "unique_token_id",
    "exp": expiration_time,
    "iat": issued_at,
    "scope": ["user", "admin", etc]
}
```

### Database Schema

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

-- Optional Profile Extension
CREATE TABLE user_profiles (
    user_id UUID PRIMARY KEY,
    -- extensible profile fields
    FOREIGN KEY (user_id) REFERENCES users(id)
);

-- Token Revocation (for security incidents)
CREATE TABLE revoked_tokens (
    token_id TEXT PRIMARY KEY,    -- jti claim
    revoked_at TIMESTAMPTZ,
    reason TEXT
);
```

### API Design (REST)

```md
Base Path: /api/v1

Authentication:
POST    /auth/register
POST    /auth/login
POST    /auth/refresh
DELETE  /auth/logout      # Revokes specific token
POST    /auth/password/reset

Users:
GET     /users/me
PATCH   /users/me
DELETE  /users/me        # Account deletion

Admin:
GET     /admin/users
POST    /admin/users/{id}/status
GET     /admin/audit-log
```

### Security Implementation

#### Request Pipeline

1. TLS Termination
2. Rate Limiting Check
3. JWT Validation
4. Permission Check
5. Input Validation
6. Handler Execution
7. Response Sanitization
8. Audit Logging

#### Logging Strategy

```rust
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
# Start PostgreSQL
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

## Production Considerations

### Deployment

- Docker containerization
- Health checks
- Graceful shutdown
- Resource limits
- Monitoring hooks

### Scaling Strategy

- Stateless design (JWT)
- Database connection pooling
- Rate limiting per instance
- Load balancer ready

### Monitoring

- Request metrics
- Error rates
- Response times
- Resource usage
- Security events
