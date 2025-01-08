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
            content.push_str(&format!(
                "{}\t{}\t{}\n",
                entry.username(),
                entry.password(),
                entry.place()
            ));
        }
        
        let encrypted = self.crypto.encrypt_to_string(content.as_bytes())?;
        fs::write(&self.file_path, encrypted).map_err(|_| "Failed to write file")?;
        
        Ok(())
    }

    pub fn load_entries(&self) -> Result<Vec<PasswordEntry>, &'static str> {
        if !Path::new(&self.file_path).exists() {
            return Ok(Vec::new());
        }

        let encrypted = fs::read_to_string(&self.file_path).map_err(|_| "Failed to read file")?;
        if encrypted.is_empty() {
            return Ok(Vec::new());
        }

        let decrypted = self.crypto.decrypt_from_string(&encrypted)?;
        let content = String::from_utf8(decrypted).map_err(|_| "Invalid UTF-8")?;

        let mut entries = Vec::new();
        for line in content.lines() {
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() != 3 {
                continue;
            }

            if let Ok(entry) = PasswordEntry::new(
                parts[0].to_string(),
                parts[1].to_string(),
                parts[2].to_string(),
            ) {
                entries.push(entry);
            }
        }

        Ok(entries)
    }
}