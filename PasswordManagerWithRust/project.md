# Password Manager Implementation Guide

## Development Order and Details

### 1. Initial Setup (Day 1-2)

1. Create new Rust project:
   ```bash
   cargo new password_manager
   cd password_manager
   ```

2. Add required dependencies to Cargo.toml:
   - rpassword: Password input
   - serde: Serialization
   - csv: CSV handling
   - aes-gcm: Encryption
   - argon2: Password hashing
   - crossterm: TUI support
   - tui: Terminal interface
   - chrono: Time handling
   - thiserror: Error handling

3. Create the basic file structure as outlined in the project plan

### 2. Core Data Structures (Day 2-3)

1. Create models/entry.rs first:
   - Implement PasswordEntry struct with all fields
   - Add validation logic for URLs and usernames
   - Implement CSV conversion methods
   - Add tests for all core functionality

2. Create crypto/encryption.rs:
   - Implement key derivation using Argon2
   - Add encryption using AES-GCM
   - Create helper methods for salt generation
   - Add comprehensive tests for crypto operations

3. Create storage/file.rs:
   - Implement basic file operations
   - Add backup functionality
   - Create vault format specification
   - Add error handling for all IO operations

### 3. Platform-Specific Code (Day 4-5)

1. Create platform/mod.rs:
   - Define the PlatformOperations trait
   - Add platform detection logic

2. Implement platform/unix.rs:
   - Add Unix-specific path handling
   - Implement file permissions
   - Add error handling for Unix systems

3. Implement platform/windows.rs:
   - Add Windows-specific path handling
   - Implement file permissions
   - Add error handling for Windows systems

### 4. CSV Operations (Day 6-7)

1. Create storage/csv.rs:
   - Implement Google Chrome format parsing
   - Add validation for required fields
   - Create export functionality
   - Add error handling for malformed CSV

2. Add tests:
   - Test with valid Chrome export
   - Test with malformed data
   - Test with missing fields
   - Test round-trip import/export

### 5. CLI Commands (Day 8-9)

1. Create cli/commands.rs:
   - Implement each command (Init, Add, Get, etc.)
   - Add input validation
   - Create help text
   - Add error messages

2. Create cli/prompt.rs:
   - Implement secure password input
   - Add confirmation dialogs
   - Create input validation
   - Add error handling

### 6. TUI Implementation (Day 10-14)

1. Create cli/tui/app.rs:
   - Implement app state management
   - Add view transitions
   - Create search functionality
   - Add error handling

2. Create cli/tui/ui.rs:
   - Implement main layout
   - Add password list view
   - Create detail view
   - Implement help screen
   - Add status bar

3. Create cli/tui/events.rs:
   - Implement keyboard handling
   - Add window resize handling
   - Create event loop
   - Add error handling

### 7. Main Application (Day 15)

1. Create main.rs:
   - Add command-line argument parsing
   - Implement config initialization
   - Create error handling
   - Add logging

## Implementation Details

### Main Entry Point (main.rs)

The main file should:
- Parse command line arguments (CLI mode vs TUI mode)
- Initialize logging
- Set up error handling
- Create platform-specific handlers
- Initialize the vault
- Start either CLI command processing or TUI

### Encryption Layer (crypto/encryption.rs)

Should implement:
- Key derivation with Argon2 (memory: 64MB, iterations: 3, parallelism: 4)
- AES-256-GCM for encryption
- Secure random number generation for salts
- Authenticated encryption for all data

### Storage Format (storage/file.rs)

Vault file format:
- Header: Magic bytes + version
- Salt: 32 bytes
- Encrypted content:
  - JSON-serialized vector of entries
  - Each entry individually encrypted
  - Authentication tag per entry

### CSV Format (storage/csv.rs)

Required fields:
- name (website/application)
- url
- username
- password
Optional fields:
- notes
- created
- modified

### Platform-Specific Details

Unix implementation:
- File permissions: 0600
- Directory permissions: 0700
- Default location: ~/.config/password_manager/
- Lock file handling

Windows implementation:
- File attributes: Hidden
- Default location: %APPDATA%\password_manager\
- Lock file handling

### TUI Layout

Main screen layout:
```
+-------------------------+
|  Password List    | Detail View  |
|  [Search Bar]     |             |
|  - Entry 1        | URL:        |
|  - Entry 2        | Username:   |
|  - Entry 3        | Password:   |
|                   | Notes:      |
+-----------------------------------+
| Status Bar  | Help: F1 | Exit: F10|
+-----------------------------------+
```

## Error Handling

Implement custom error types for:
- Crypto operations
- File operations
- CSV parsing
- User input
- Platform-specific operations

Each error should:
- Include context
- Be user-friendly
- Support error chaining
- Include file/line information in debug mode

## Testing Strategy

1. Unit tests for:
   - Encryption/decryption
   - Password entry validation
   - CSV parsing
   - File operations

2. Integration tests for:
   - Full vault operations
   - Import/export
   - Platform-specific features

3. End-to-end tests for:
   - CLI commands
   - TUI operations
   - Error scenarios


## File Structure with Contents

```rust
src/
├── main.rs                # Application entry
│   fn main() {
│       // Parse command line args
│       // Initialize app state
│       // Start TUI or handle CLI commands
│       // Setup error handling
│   }
│
├── models/
│   ├── mod.rs            # Export models
│   │   pub mod entry;
│   │   pub use entry::PasswordEntry;
│   │
│   └── entry.rs          # Core data structures
│       pub struct PasswordEntry {
│           url: String,
│           username: String,
│           password: Vec<u8>,  // Encrypted
│           notes: Option<String>,
│           created_at: DateTime<Utc>,
│           modified_at: DateTime<Utc>
│       }
│       
│       impl PasswordEntry {
│           fn new() -> Self
│           fn validate(&self) -> Result<()>
│           fn update_password(&mut self, new_pass: &[u8])
│           fn to_csv_record(&self) -> Record
│           fn from_csv_record(record: Record) -> Result<Self>
│       }
│
├── crypto/
│   ├── mod.rs            # Export crypto
│   │   pub mod encryption;
│   │   pub use encryption::*;
│   │
│   └── encryption.rs     # Encryption operations
│       fn derive_key(password: &str, salt: &[u8]) -> Vec<u8>
│       fn encrypt_password(password: &[u8], key: &[u8]) -> Result<Vec<u8>>
│       fn decrypt_password(encrypted: &[u8], key: &[u8]) -> Result<Vec<u8>>
│       fn generate_salt() -> [u8; 32]
│
├── storage/
│   ├── mod.rs            # Export storage
│   │   pub mod file;
│   │   pub mod csv;
│   │   pub use file::VaultStorage;
│   │
│   ├── file.rs           # File operations
│   │   pub struct VaultStorage {
│   │       path: PathBuf,
│   │       master_key: Vec<u8>,
│   │   }
│   │
│   │   impl VaultStorage {
│   │       fn new(path: PathBuf, master_key: Vec<u8>) -> Self
│   │       fn read_vault(&self) -> Result<Vec<PasswordEntry>>
│   │       fn write_vault(&self, entries: &[PasswordEntry]) -> Result<()>
│   │       fn backup_vault(&self) -> Result<()>
│   │   }
│   │
│   └── csv.rs            # CSV handling
│       fn import_entries(path: &Path) -> Result<Vec<PasswordEntry>>
│       fn export_entries(entries: &[PasswordEntry], path: &Path) -> Result<()>
│       fn validate_csv_headers(headers: &[String]) -> Result<()>
│
├── platform/
│   ├── mod.rs            # Platform traits
│   │   pub trait PlatformOperations {
│   │       fn get_config_path() -> PathBuf;
│   │       fn create_secure_file(path: &Path) -> Result<()>;
│   │       fn read_secure_file(path: &Path) -> Result<Vec<u8>>;
│   │       fn write_secure_file(path: &Path, data: &[u8]) -> Result<()>;
│   │   }
│   │
│   ├── unix.rs           # Unix implementation
│   │   pub struct UnixOperations;
│   │   
│   │   impl PlatformOperations for UnixOperations {
│   │       fn get_config_path() -> PathBuf { "~/.config/password_manager" }
│   │       // Other implementations
│   │   }
│   │
│   └── windows.rs        # Windows implementation
│       pub struct WindowsOperations;
│       
│       impl PlatformOperations for WindowsOperations {
│           fn get_config_path() -> PathBuf { "%APPDATA%\password_manager" }
│           // Other implementations
│       }
│
└── cli/
    ├── mod.rs            # Export CLI/TUI
    │   pub mod commands;
    │   pub mod prompt;
    │   pub mod tui;
    │
    ├── commands.rs       # Command implementations
    │   pub enum Command {
    │       Init,
    │       Add,
    │       Get(String),
    │       List,
    │       Edit(String),
    │       Delete(String),
    │       Import(PathBuf),
    │       Export(PathBuf),
    │       ChangeMaster,
    │   }
    │
    │   impl Command {
    │       fn execute(&self, vault: &mut VaultStorage) -> Result<()>
    │   }
    │
    ├── prompt.rs         # User input
    │   fn read_password() -> Result<String>
    │   fn confirm_action(msg: &str) -> bool
    │   fn get_input(prompt: &str) -> Result<String>
    │
    └── tui/              # TUI components
        ├── mod.rs        # Export TUI
        │   pub mod app;
        │   pub mod ui;
        │   pub mod events;
        │
        ├── app.rs        # App state
        │   pub struct App {
        │       entries: Vec<PasswordEntry>,
        │       current_view: View,
        │       selected_index: Option<usize>,
        │       search_term: String,
        │       error_message: Option<String>,
        │   }
        │
        │   impl App {
        │       fn new() -> Self
        │       fn handle_input(&mut self, key: Key) -> Result<()>
        │       fn search(&mut self, term: &str)
        │       fn select_entry(&mut self, index: usize)
        │   }
        │
        ├── ui.rs         # UI rendering
        │   fn draw(app: &App, frame: &mut Frame)
        │   fn draw_list(entries: &[PasswordEntry], frame: &mut Frame)
        │   fn draw_detail(entry: &PasswordEntry, frame: &mut Frame)
        │   fn draw_help(frame: &mut Frame)
        │   fn draw_status_bar(app: &App, frame: &mut Frame)
        │
        └── events.rs     # Event handling
            fn handle_events(app: &mut App) -> Result<()>
            fn handle_keyboard(app: &mut App, key: Key) -> Result<()>
            fn handle_resize(app: &mut App, width: u16, height: u16)
```