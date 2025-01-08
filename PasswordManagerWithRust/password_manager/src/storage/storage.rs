use std::fs::{self, File};
use std::io::{self};
use std::path::Path;

use crate::crypto::FileCrypto;
use crate::models::PasswordEntry;

pub struct FileStorage {
    file_path: String,
    crypto: FileCrypto,
}

impl FileStorage {
    pub fn new(file_path: String, master_password: &str) -> Self {
        Self {
            file_path,
            crypto: FileCrypto::new(master_password),
        }
    }

    pub fn initialize(&self) -> io::Result<()> {
        if !Path::new(&self.file_path).exists() {
            let file = File::create(&self.file_path)?;
            file.set_len(0)?;
        }
        Ok(())
    }

    pub fn save_entries(&self, entries: &[PasswordEntry]) -> Result<(), &'static str> {
        let mut content = String::new();
        for entry in entries {
            let encrypted_username = self.crypto.encrypt_to_string(entry.username().as_bytes())?;
            let encrypted_password = self.crypto.encrypt_to_string(entry.password().as_bytes())?;
            let encrypted_place = self.crypto.encrypt_to_string(entry.place().as_bytes())?;
            content.push_str(&format!(
                "{}\t{}\t{}\n",
                encrypted_username,
                encrypted_password,
                encrypted_place
            ));
        }
        
        fs::write(&self.file_path, content).map_err(|_| "Failed to write file")?;
        
        Ok(())
    }

    pub fn load_entries(&self) -> Result<Vec<PasswordEntry>, &'static str> {
        if !Path::new(&self.file_path).exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(&self.file_path).map_err(|_| "Failed to read file")?;
        if content.is_empty() {
            return Ok(Vec::new());
        }

        let mut entries = Vec::new();
        for line in content.lines() {
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() != 3 {
                continue;
            }

            let username = self.crypto.decrypt_from_string(parts[0])?;
            let password = self.crypto.decrypt_from_string(parts[1])?;
            let place = self.crypto.decrypt_from_string(parts[2])?;

            if let Ok(entry) = PasswordEntry::new(
                String::from_utf8(username).map_err(|_| "Invalid UTF-8")?,
                String::from_utf8(password).map_err(|_| "Invalid UTF-8")?,
                String::from_utf8(place).map_err(|_| "Invalid UTF-8")?,
            ) {
                entries.push(entry);
            }
        }

        Ok(entries)
    }
}