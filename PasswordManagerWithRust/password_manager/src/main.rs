use std::io::{self, Write};
use std::process;

mod crypto;
mod models;
mod storage;

use models::PasswordEntry;
use storage::FileStorage;

const PASSWORD_FILE: &str = "passwords.enc";

fn get_master_password() -> String {
    print!("Enter master password: ");
    io::stdout().flush().unwrap();
    
    let mut password = String::new();
    io::stdin().read_line(&mut password).unwrap();
    password.trim().to_string()
}

fn print_menu() {
    println!("\nPassword Manager Menu:");
    println!("1. Add new password");
    println!("2. List all passwords");
    println!("3. Search passwords");
    println!("4. Exit");
    print!("\nChoice: ");
    io::stdout().flush().unwrap();
}

fn add_password(storage: &FileStorage, entries: &mut Vec<PasswordEntry>) -> Result<(), &'static str> {
    print!("Enter username: ");
    io::stdout().flush().unwrap();
    let mut username = String::new();
    io::stdin().read_line(&mut username).unwrap();

    print!("Enter password: ");
    io::stdout().flush().unwrap();
    let mut password = String::new();
    io::stdin().read_line(&mut password).unwrap();

    print!("Enter place (e.g., 'gmail', 'github'): ");
    io::stdout().flush().unwrap();
    let mut place = String::new();
    io::stdin().read_line(&mut place).unwrap();

    let entry = PasswordEntry::new(
        username.trim().to_string(),
        password.trim().to_string(),
        place.trim().to_string(),
    )?;

    entries.push(entry);
    storage.save_entries(entries)?;
    println!("Password added successfully!");
    Ok(())
}

fn list_passwords(entries: &[PasswordEntry]) {
    if entries.is_empty() {
        println!("No passwords stored.");
        return;
    }

    println!("\nStored Passwords:");
    println!("{:<20} {:<20} {:<20}", "Username", "Password", "Place");
    println!("{}", "-".repeat(60));

    for entry in entries {
        println!(
            "{:<20} {:<20} {:<20}",
            entry.username(),
            entry.password(),
            entry.place()
        );
    }
}

fn search_passwords(entries: &[PasswordEntry]) {
    print!("Enter search term: ");
    io::stdout().flush().unwrap();
    let mut term = String::new();
    io::stdin().read_line(&mut term).unwrap();
    let term = term.trim().to_lowercase();

    let results: Vec<&PasswordEntry> = entries
        .iter()
        .filter(|e| {
            e.username().to_lowercase().contains(&term)
                || e.place().to_lowercase().contains(&term)
        })
        .collect();

    if results.is_empty() {
        println!("No matching passwords found.");
        return;
    }

    println!("\nSearch Results:");
    println!("{:<20} {:<20} {:<20}", "Username", "Password", "Place");
    println!("{}", "-".repeat(60));

    for entry in results {
        println!(
            "{:<20} {:<20} {:<20}",
            entry.username(),
            entry.password(),
            entry.place()
        );
    }
}

fn main() {
    let master_password = get_master_password();
    let storage = FileStorage::new(PASSWORD_FILE.to_string(), &master_password);

    if let Err(e) = storage.initialize() {
        eprintln!("Failed to initialize storage: {}", e);
        process::exit(1);
    }

    let mut entries = match storage.load_entries() {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Failed to load entries: {}", e);
            process::exit(1);
        }
    };

    loop {
        print_menu();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                if let Err(e) = add_password(&storage, &mut entries) {
                    eprintln!("Error adding password: {}", e);
                }
            }
            "2" => list_passwords(&entries),
            "3" => search_passwords(&entries),
            "4" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice, please try again."),
        }
    }
}