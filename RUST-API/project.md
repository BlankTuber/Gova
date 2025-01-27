# User API Project Plan

## Roadmap

### Phase 1: Initial Setup

1. **Set up the project environment**:
   - Initialize a new Rust project using `cargo`.
   - Configure the `Cargo.toml` file with the listed dependencies.
   - Set up `.env` for environment variables.
2. **Database integration**:
   - Choose a database (e.g., PostgreSQL, MySQL, etc.).
   - Define the database schema for users (e.g., tables for users, roles, etc.).
   - Connect to the database using `sqlx` and verify connectivity.
   - Set up migrations using `sqlx migrate`.
3. **Basic API structure**:
   - Implement a basic Rocket server setup.
   - Create a health check endpoint (e.g., `/health`).

### Phase 2: Core Functionality

1. **User Authentication**:
   - Implement JWT-based authentication using `jsonwebtoken`.
   - Add endpoints in the following order:
     - **Register**: Create a new user and store hashed passwords.
     - **Login**: Validate credentials and generate a JWT.
     - **Get User**: Fetch user details using authentication middleware.
   - Validate input using `validator`.
2. **User Management**:
   - Add endpoints for additional CRUD operations (e.g., update user, delete user).
   - Ensure proper validation and error handling.
3. **Email integration**:
   - Configure `lettre` for sending emails (e.g., welcome email, password reset).
   - Create email templates.

### Phase 3: Security and Enhancements

1. **Role-based Access Control (RBAC)**:
   - Add roles and permissions for users.
   - Restrict access to certain endpoints based on roles.
2. **Input Validation and Sanitization**:
   - Use `validator` to ensure data integrity.
   - Sanitize all user inputs to prevent injection attacks.
3. **Middleware**:
   - Implement middleware for authentication (JWT validation).
   - Create reusable middleware for logging and request validation.

### Phase 4: Optimization and Documentation

1. **Code Optimization**:
    - Refactor code for better modularity and readability.
    - Use Postman or a similar tool for manual API testing.
2. **Documentation**:
    - Document the API endpoints and expected input/output.
    - Add comments to the codebase for maintainability.
3. **Deployment**:
    - Deploy the API locally, ensuring proper configuration for the environment.

---

## Project / Folder Structure

```md
userspace/
├── src/
│   ├── main.rs         # Entry point of the application
│   ├── lib.rs          # Main library file (optional if organizing modules)
│   ├── config/         # Configuration-related code
│   │   ├── mod.rs      # Module file
│   │   └── database.rs # Database connection setup
│   ├── routes/         # API route handlers
│   │   ├── mod.rs      # Module file
│   │   ├── auth/       # Authentication routes
│   │   │   ├── mod.rs  # Module file
│   │   │   ├── register.rs # Register route
│   │   │   ├── login.rs    # Login route
│   │   │   └── get_user.rs # Get user route
│   │   └── users/      # User management routes
│   │       ├── mod.rs  # Module file
│   │       ├── create.rs  # Create user
│   │       ├── update.rs  # Update user
│   │       ├── delete.rs  # Delete user
│   │       └── get_all.rs # Get all users
│   ├── controllers/    # Business logic and services
│   │   ├── mod.rs      # Module file
│   │   ├── auth/       # Authentication controllers
│   │   │   ├── mod.rs  # Module file
│   │   │   ├── register.rs # Register controller
│   │   │   ├── login.rs    # Login controller
│   │   │   └── get_user.rs # Get user controller
│   │   └── users/      # User management controllers
│   │       ├── mod.rs  # Module file
│   │       ├── create.rs  # Create user logic
│   │       ├── update.rs  # Update user logic
│   │       ├── delete.rs  # Delete user logic
│   │       └── get_all.rs # Get all users logic
│   ├── models/         # Data models
│   │   ├── mod.rs      # Module file
│   │   └── user.rs     # User model
│   ├── utils/          # Utility functions and helpers
│   │   ├── mod.rs      # Module file
│   │   ├── validation.rs # Validation functions
│   │   └── security.rs # Security helpers (e.g., hashing, sanitization)
│   ├── middleware/     # Middleware components
│   │   ├── mod.rs      # Module file
│   │   ├── auth.rs     # JWT authentication middleware
│   │   └── logging.rs  # Logging middleware
├── migrations/         # SQL migration files for database schema
├── tests/              # Integration tests (optional)
├── .env                # Environment variables
├── Cargo.toml          # Rust project configuration
├── README.md           # Project overview and documentation
```

---

## Explanation of Key Components

### Middleware

- **Authentication Middleware**: Verifies JWT tokens and attaches user information to the request.
- **Logging Middleware**: Logs incoming requests and their responses for debugging purposes.

### Security, Validation, and Sanitization

- **Security**:
  - Use `bcrypt` or `argon2` for password hashing.
  - Always sanitize user inputs to prevent SQL injection and XSS attacks.
  - Enable HTTPS for secure communication.
- **Validation**:
  - Use `validator` to enforce input formats (e.g., email, password strength).
  - Implement custom validation logic in `utils/validation.rs`.
- **Sanitization**:
  - Remove or escape harmful characters in inputs.
  - Ensure database queries are parameterized.

### Migrations

1. Use `sqlx migrate` to create and manage migrations:
   - Run `sqlx migrate add <migration_name>` to create a new migration file.
   - Write SQL scripts for creating or modifying tables in the migration file.
   - Apply migrations with `sqlx migrate run`.
2. Maintain version control over migration files to track schema changes.

### Testing with Postman

- Use Postman to manually test API endpoints.
- Create a Postman collection for organizing requests.
- Use environment variables in Postman for dynamic testing (e.g., base URL, tokens).
