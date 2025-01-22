# Simplified User System Project Guide

## Project Structure
```md
.
├── src/
│   ├── main.rs                # Entry point
│   ├── routes/
│   │   ├── mod.rs             # Routes module declaration
│   │   ├── user.rs            # Basic user operations
│   │   ├── auth.rs            # Authentication & security
│   │   └── admin.rs           # Admin & moderation
│   ├── models/
│   │   ├── mod.rs             # Models module declaration
│   │   ├── user.rs            # User model
│   │   ├── session.rs         # Session management
│   │   └── audit.rs           # Basic audit logging
│   ├── db/
│   │   ├── mod.rs             # DB module declaration
│   │   └── mongo.rs           # MongoDB operations
│   ├── services/
│   │   ├── mod.rs             # Services module declaration
│   │   ├── auth.rs            # Authentication logic
│   │   ├── email.rs           # Email notifications
│   │   └── moderation.rs      # Moderation actions
│   └── config/
│       ├── mod.rs             # Config module declaration
│       └── settings.rs        # App configuration
└── Cargo.toml                 # Project dependencies
```

## Development Roadmap

### Phase 1: Project Setup and Basic User Operations
1. Learn Rust basics (ownership, borrowing, lifetimes)
2. Set up project with Rocket and MongoDB
3. Implement basic user model
4. Create CRUD endpoints for users
5. Add basic error handling

### Phase 2: Authentication Foundation
1. Implement password hashing
2. Create JWT handling
3. Basic login/register endpoints
4. Session management
5. Email verification system

### Phase 3: Moderation and Admin
1. Admin user roles and permissions
2. User management for admins
   - Ban/unban users
   - Reset user passwords
   - View user details
3. Basic audit logging
4. Simple admin dashboard endpoints

## Essential Dependencies
```toml
[dependencies]
rocket = "0.5"
mongodb = "2.8"
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full"] }
jsonwebtoken = "9.2"
argon2 = "0.5"
lettre = "0.11"      # For email sending
validator = "0.16"
chrono = "0.4"       # For audit timestamps
```

## Learning Resources

1. **Rust Fundamentals**
   - The Rust Book (official documentation)
   - Rust by Example

2. **Rocket Framework**
   - Official Rocket Guide
   - Rocket Example Applications

3. **MongoDB with Rust**
   - MongoDB Rust Driver Documentation
   - MongoDB CRUD Operations Guide

## Basic Admin Features

1. **User Management**
   - View all users
   - Ban/unban users
   - Reset user passwords
   - Update user roles

2. **Audit System**
   - Log admin actions
   - View action history
   - Track user status changes

3. **Access Control**
   - Role-based permissions
   - Admin-only routes
   - Moderator capabilities