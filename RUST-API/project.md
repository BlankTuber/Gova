# Project / Folder Structure

```md
userspace/
├── src/
│   ├── main.rs
│   ├── config/
│   │   ├── mod.rs
│   │   └── database.rs
│   ├── routes/
│   │   ├── mod.rs
│   │   ├── auth/
│   │   │   ├── mod.rs
│   │   │   ├── register.rs
│   │   │   ├── login.rs
│   │   │   └── ...
│   │   └── users/
│   │       ├── mod.rs
│   │       ├── update.rs
│   │       ├── delete.rs
│   │       ├── get_user.rs
│   │       ├── ...
│   ├── controllers/
│   │   ├── mod.rs
│   │   ├── auth/
│   │   │   ├── mod.rs
│   │   │   ├── register.rs
│   │   │   ├── login.rs
│   │   │   └── ...
│   │   └── users/
│   │       ├── mod.rs
│   │       ├── update.rs
│   │       ├── delete.rs
│   │       ├── get_user.rs
│   │       ├── ...
│   ├── models/
│   │   ├── mod.rs
│   │   └── log.rs
│   │   └── user.rs
│   │   └── role.rs
│   │   └── permission.rs
│   │   └── role_permissions.rs
│   │   └── user_permissions.rs
│   │   └── user_roles.rs
│   │   └── ...
│   ├── utils/
│   │   ├── mod.rs
│   │   ├── validation.rs
│   │   └── create_jwt.rs
│   │   └── hashing.rs
│   │   └── ...
│   ├── middleware/
│   │   ├── mod.rs
│   │   ├── auth.rs
│   │   └── logging.rs
│   │   └── api_key_check.rs
├── .env
├── Cargo.toml
├── Rocket.toml
└── README.md
```
