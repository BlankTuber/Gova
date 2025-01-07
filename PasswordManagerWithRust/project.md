# Password Manager Implementation Plan

## Core Requirements

- CSV import/export (Google format compatible)
- Password encryption/decryption
- Interactive TUI with keyboard navigation
- Local encrypted storage
- Cross-platform support (Windows/Linux)

## Project Roadmap

### Phase 1: Core Setup (1 week)

- Basic project structure
- Data structures for password entries
- File path handling for both platforms
- Master password setup and validation

### Phase 2: CSV Operations (1 week)

- Import system
  - Google Chrome format parsing
  - Basic validation
  - Error handling
- Export system
  - Google Chrome format output
  - Basic field mapping

### Phase 3: Encryption (1 week)

- Master key derivation
- Password encryption/decryption
- Secure storage format
- Basic file operations

### Phase 4: CLI and TUI (1-2 weeks)

- Basic commands (CLI):
  - init: Create vault
  - add: New entry
  - get: Retrieve entry
  - list: Show entries
  - edit: Modify entry
  - delete: Remove entry
  - import/export: CSV handling
  - change-master: Update master password

- Interactive Interface (TUI):
  - Main menu with arrow key navigation
  - Password list with search/filter
  - Entry viewer/editor with form navigation
  - Interactive import/export wizard
  - Status bar with shortcuts

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

## Cross-Platform Considerations

### File System

- Path separators (\ vs /)
- Storage locations:
  - Linux: ~/.config/password_manager/
  - Windows: %APPDATA%\password_manager\

### Platform-Specific Code

- File operations
- Path handling
- Error handling

### TUI Considerations

- Terminal size differences
- Key code mappings
- Color support
- Unicode support

This implementation provides:

1. Secure password storage
2. Easy-to-use TUI interface
3. Platform independence
4. CSV import/export compatibility
5. Extensible architecture

The modular design allows for easy addition of features while maintaining a clean separation of concerns between UI, business logic, and platform-specific code.
