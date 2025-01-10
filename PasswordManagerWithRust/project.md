# Simplified Password Manager Implementation Guide

## Core Dependencies

- csv: For import/export functionality
- chrono: For timestamp handling
- rust_crypto: For encryption operations

## Project Structure

```md
src/
├── main.rs              # Application entry point
├── models/
│   ├── mod.rs           # Module exports
│   └── entry.rs         # Password entry structure
├── crypto/
│   ├── mod.rs           # Module exports
│   └── encryption.rs    # Encryption operations
└── storage/
    ├── mod.rs           # Module exports
    ├── file.rs          # File operations
    └── csv.rs           # CSV import/export
```

## Implementation Plan

### Phase 1: Core Data Structure (Days 1-2)

1. Set up project and dependencies:
   - Create new project with `cargo new`
   - Add minimal dependencies to Cargo.toml
   - Set up basic error handling structure

2. Implement password entry structure:
   - Define fields: username, password, url
   - Create constructors and validation methods
   - Implement basic CRUD operations
   - Add comprehensive tests for entry operations

3. Create basic file operations:
   - Define vault file format specification
   - Implement file reading and writing
   - Add backup functionality
   - Create error types for file operations

### Phase 2: Encryption Layer (Days 3-4)

1. Set up encryption operations:
   - Implement key derivation from master password
   - Create salt generation functionality
   - Add file encryption/decryption operations
   - Write tests for crypto operations

2. Integrate encryption with storage:
   - Add encryption to vault writing
   - Add decryption to vault reading
   - Implement secure memory handling
   - Test file encryption/decryption

### Phase 3: CSV Operations (Days 5-6)

1. Basic CSV functionality:
   - Define CSV format specification
   - Implement CSV reading
   - Add CSV writing capabilities
   - Create CSV validation

2. Advanced CSV operations:
   - Add import from common password manager formats
   - Implement export functionality
   - Create data validation and cleaning
   - Write tests for CSV operations

### Phase 4: Command Line Interface (Days 7-8)

1. Basic CLI operations:
   - Implement argument parsing
   - Add basic CRUD commands
   - Create help documentation
   - Add error messages

2. Advanced CLI features:
   - Implement secure password input
   - Add confirmation dialogs
   - Create search functionality
   - Add import/export commands

## File Format Specifications

### Vault File Format

1. Header (16 bytes):
   - Magic number (8 bytes)
   - Version number (4 bytes)
   - Reserved (4 bytes)

2. Encryption Metadata (48 bytes):
   - Salt (32 bytes)
   - IV (16 bytes)

3. Encrypted Data Block:
   - Serialized JSON containing all entries
   - Single encryption for entire data block

### CSV Format

Required fields:

- name
- username
- password
- url

Optional fields:

- notes
- created_at
- modified_at

## Testing Strategy

### 1. Unit Tests

- Entry validation
- Encryption/decryption
- File operations
- CSV parsing

### 2. Integration Tests

- Full vault operations
- Import/export functionality
- Command line operations

### 3. Error Cases

- Invalid passwords
- Corrupted files
- Malformed CSV
- Permission issues

## Security Considerations

1. Master Password:
   - Minimum length enforcement
   - Complexity requirements
   - Secure input handling

2. File Security:
   - Secure file permissions
   - Temporary file handling
   - Backup file protection

3. Memory Security:
   - Secure password handling
   - Memory wiping
   - Buffer security

## Error Handling Strategy

1. Custom error types for:
   - File operations
   - Encryption operations
   - CSV operations
   - Validation errors

2. Error context:
   - Descriptive messages
   - Error chain support
   - Recovery suggestions

## Future Expansion Considerations

1. TUI Implementation:
   - Separation of concerns for future TUI
   - Abstracted data operations
   - Event handling preparation

2. Additional Features:
   - Password generation
   - Password strength analysis
   - Category management
   - Tags and metadata
