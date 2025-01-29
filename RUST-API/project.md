# User API Project Plan

## Project / Folder Structure

```md
userspace/
├── src/
│   ├── main.rs         # Single entry point of the application
│   ├── config/         # Configuration-related code
│   │   ├── mod.rs          # Module file re-exporting anything in config/
│   │   └── database.rs     # Database connection setup (pool init, etc.)
│   ├── routes/         # API route handlers (thin, handle HTTP specifics)
│   │   ├── mod.rs          # Module file re-exporting routes
│   │   ├── auth/           # Authentication routes
│   │   │   ├── mod.rs          # Module file
│   │   │   ├── register.rs     # Register route
│   │   │   ├── login.rs        # Login route
│   │   │   └── get_user.rs     # Get user route
│   │   └── users/          # User management routes
│   │       ├── mod.rs          # Module file
│   │       ├── update.rs       # Update user route
│   │       ├── delete.rs       # Delete user route
│   ├── controllers/        # Business logic and services (the "meat" of the app)
│   │   ├── mod.rs              # Module file re-exporting controllers
│   │   ├── auth/               # Authentication controllers
│   │   │   ├── mod.rs              # Module file
│   │   │   ├── register.rs         # Register logic
│   │   │   ├── login.rs            # Login logic
│   │   │   └── get_user.rs         # Get user logic
│   │   └── users/              # User management controllers
│   │       ├── mod.rs              # Module file
│   │       ├── update.rs           # Update user logic
│   │       ├── delete.rs           # Delete user logic
│   ├── models/                 # Data models
│   │   ├── mod.rs                  # Module file re-exporting model files
│   │   ├── schema.rs              # Diesel's schema definitions (auto-generated or manually created)
│   │   └── user.rs                # User model (e.g. structs, Diesel annotations)
│   ├── utils/                  # Utility functions and helpers
│   │   ├── mod.rs                  # Module file re-exporting utils
│   │   ├── validation.rs           # Validation functions (e.g. using validator crate)
│   │   └── createJWT.rs             # JWT generation
│   │   └── hashing.rs             # Hashing
│   │   └── security.rs             # Security helpers
│   ├── middleware/             # Middleware components (e.g., Rocket fairings, guards)
│   │   ├── mod.rs                  # Module file re-exporting middleware
│   │   ├── auth.rs                 # JWT authentication guard/fairing
│   │   └── logging.rs              # Logging or request tracing
├── migrations/                 # Diesel migration files for database schema
├── tests/                      # Integration tests (optional)
├── .env                        # Environment variables (dev only)
├── Cargo.toml                  # Rust project configuration
└── README.md                   # Project overview and documentation
```

## 1. **Configure the Database**

1. **Set up Diesel**  

   ```bash
      diesel setup
   ```  

   This command usually looks for `DATABASE_URL` in `.env` or the environment. Make sure you have `.env` with something like:  

   ```md
      DATABASE_URL=postgres://postgres:password@localhost:5432/mydb
   ```  

   Diesel will create a `diesel.toml` file and maybe set up initial migrations.

2. **Create your first migration**  

   ```bash
      diesel migration generate create_users
   ```  

   Fills `/migrations/` with a timestamped folder containing `up.sql` and `down.sql`.  
   - Fill in `up.sql`: e.g., `CREATE TABLE users (...)`.  
   - Fill in `down.sql`: e.g., `DROP TABLE users;`.

3. **Run the migration**  

   ```bash
      diesel migration run
   ```  

   This applies the SQL to your local database.

4. **Generate `schema.rs`**  

   ```bash
      diesel print-schema > src/models/schema.rs
   ```  

   This file includes Diesel’s `table! { ... }` macros.  
   - If you make schema changes later, re-run this command to keep `schema.rs` in sync.

---

## 2. **Define Models**

1. **Create `user.rs`** in `models/`  
   - Write your `#[derive(Queryable, Insertable)]` structs (`User`, `NewUser`, etc.).  
   - Reference the `schema::users` table from `schema.rs`.  
   - Optionally add `#[derive(Serialize, Deserialize)]` if you plan to convert to/from JSON.  

2. **Check everything compiles**  
   - Run `cargo check`.  
   - Fix any errors related to Diesel macros or missing references.

---

## 3. **Database Connection Setup**

1. **`config/database.rs`**  
   - Write a function (e.g., `init_pool(database_url: &str)`) that uses Diesel’s `r2d2` connection pool.  
   - Return a pool (e.g., `Pool<ConnectionManager<PgConnection>>`).  

2. **Initialize the pool in `main.rs`**  
   - Load environment variables (`dotenv::dotenv().ok();`).  
   - Read `DATABASE_URL` from `std::env::var(...)`.  
   - Call `init_pool(...)` to create the pool.  
   - Pass it to Rocket with `.manage(pool)`.

---

## 4. **Set Up Routes and Controllers**

1. **Create Route Handlers** in `routes/`  
   - For example, `routes/auth/register.rs` with a function like:

     ```rust
      #[post("/register", data = "<user_data>")]
      pub fn register(user_data: Json<NewUser>, pool: &State<DbPool>) -> Result<Status, SomeError> {
            // ... call controller logic ...
      }
     ```

   - Keep them **thin**: parse requests, call controllers, return a response.

2. **Create Business Logic** in `controllers/`  
   - For example, `controllers/auth/register.rs` might have a function `register_user(...)` that does validations, hashing, inserting into the DB.  
   - This function returns a result that your route layer can translate into an HTTP response.

3. **Mount Routes** in `main.rs**  

   ```rust
      rocket::build()
         .manage(pool)
         .mount("/auth", routes![register, login])
         .mount("/users", routes![update_user, delete_user]);
   ```

---

## 5. **Utility Functions**

1. **Validation** in `utils/validation.rs`  
   - Functions or structs with `#[derive(Validate)]` from `validator` crate.  
   - Possibly used by your controllers (e.g., `register_user`) to validate input.

2. **Security** in `utils/security.rs` or multiple files  
   - `createJWT.rs` for generating JWT tokens (using `jsonwebtoken`).  
   - `hashing.rs` for hashing passwords (using `argon2` or `bcrypt`, etc.).

---

## 6. **Middleware / Fairings / Guards**

1. **Auth Middleware** (`middleware/auth.rs`)  
   - For Rocket, you might create a [Request Guard](https://rocket.rs/v0.5-rc/guide/requests/#guards) that checks a JWT in the `Authorization` header.  
   - Or a [Fairing](https://rocket.rs/v0.5-rc/guide/fairings/) that runs code before/after requests.  

2. **Logging** (`middleware/logging.rs`)  
   - Another Fairing or something that logs requests/responses.

3. **Enable them** in `main.rs`  
   - Something like `.attach(MyLoggingFairing)` or route-level guards.

---

## 7. **Testing**

1. **Integration Tests** in `/tests/`  
   - You can spin up a Rocket instance pointing to a test DB.  
   - Make requests to your routes (via `rocket::local::blocking::Client` or `reqwest`) and assert responses.

2. **Unit Tests** near each module  
   - For example, inside `controllers/auth/register.rs`, add `#[cfg(test)] mod tests { ... }` to test the registration logic directly.

---

## 8. **Iterate and Refine**

- **Add new migrations** as you evolve the schema.  
- **Regenerate `schema.rs`** after each DB change.  
- **Update models** and controllers accordingly.  
- **Expand routes** or add new modules (e.g., roles, permissions).  
- **Optimize** or tune your code, queries, and error handling.

---

## 9. **Deployment Considerations** (Optional)

- **Production environment**: provide `DATABASE_URL` via system environment variables (Docker, systemd, etc.).  
- **Rocket** typically runs behind a reverse proxy like Nginx, or you can run it standalone.  
- **Security**: Ensure HTTPS, secure JWT secrets, password hashing, etc.

---

### Summary

1. **Initialize** project & set up dependencies.  
2. **Configure DB** (Diesel CLI, migrations, `schema.rs`).  
3. **Define models** and set up a **DB connection pool**.  
4. **Write controllers** (business logic) and **routes** (thin HTTP handlers).  
5. **Add utility modules** (validation, JWT, hashing) and **middleware** (auth guards, logging).  
6. **Test** your API with unit/integration tests.  
7. **Iterate** on migrations, schema, and code.  
8. **Deploy** securely to production when ready.
