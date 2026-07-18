use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use std::path::Path;

#[derive(Parser)]
#[command(name = "meincms_admin")]
#[command(about = "MeinCMS Administrator & User Management Tool (Rust Edition)", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Erstellt einen neuen Administrator-Benutzer
    CreateUser {
        /// E-Mail / Benutzername des Benutzers
        #[arg(short, long)]
        username: Option<String>,
    },
    /// Listet alle Administratoren auf
    ListUsers,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminUser {
    pub username: String,
    pub password_hash: String,
    pub role: String,
    pub email_confirmed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserStore {
    pub users: Vec<AdminUser>,
}

impl UserStore {
    pub fn load(path: &str) -> Self {
        if Path::new(path).exists() {
            if let Ok(content) = std::fs::read_to_string(path) {
                if let Ok(store) = serde_json::from_str(&content) {
                    return store;
                }
            }
        }
        Self::default()
    }

    pub fn save(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }
}

fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string()
}

fn verify_password(password: &str, hash_str: &str) -> bool {
    if let Ok(parsed_hash) = PasswordHash::new(hash_str) {
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
    } else {
        false
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let store_path = "config/users.json";
    let mut store = UserStore::load(store_path);

    if let Some(command) = cli.command {
        match command {
            Commands::CreateUser { username } => {
                let uname = match username {
                    Some(u) => u,
                    None => prompt_input("Benutzername (E-Mail): "),
                };

                println!("Passwort eingeben:");
                let password = rpassword::read_password()?;

                if password.len() < 12 {
                    println!("[!] Fehler: Das Passwort muss mindestens 12 Zeichen lang sein.");
                    return Ok(());
                }

                let p_hash = hash_password(&password);
                store.users.push(AdminUser {
                    username: uname.clone(),
                    password_hash: p_hash,
                    role: "Admin".to_string(),
                    email_confirmed: true,
                });

                store.save(store_path)?;
                println!("[OK] ERFOLG: Administrator '{}' wurde erstellt.", uname);
            }
            Commands::ListUsers => {
                println!("\n--- Aktuelle Administratoren ---");
                if store.users.is_empty() {
                    println!("Keine Benutzer gefunden.");
                } else {
                    for u in &store.users {
                        println!("- {} (Rolle: {})", u.username, u.role);
                    }
                }
            }
        }
        return Ok(());
    }

    // Interaktives Menü (wie in UserAdmin/Program.cs)
    run_interactive_menu(&mut store, store_path)?;
    Ok(())
}

fn run_interactive_menu(
    store: &mut UserStore,
    store_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if store.users.is_empty() {
        println!("\n--- Notfall-Administrator Registrierung ---");
        println!("Kein Administrator gefunden. Bitte legen Sie den Notfall-Account an.");

        let uname = prompt_input("Admin Benutzername (E-Mail) : ");
        if uname.trim().is_empty() {
            return Ok(());
        }

        print!("Admin Passwort             : ");
        io::stdout().flush()?;
        let password = rpassword::read_password()?;

        if password.len() < 12 {
            println!("\n[!] Fehler: Passwort muss mindestens 12 Zeichen lang sein.");
            return Ok(());
        }

        let p_hash = hash_password(&password);
        store.users.push(AdminUser {
            username: uname.clone(),
            password_hash: p_hash,
            role: "Admin".to_string(),
            email_confirmed: true,
        });
        store.save(store_path)?;

        println!("\nERFOLG: Der Administrator wurde erstellt und die Rolle zugewiesen.");
        return Ok(());
    }

    loop {
        println!("\n--- Administrator & User Management (Rust Edition) ---");
        println!("Aktuelle Administratoren ({})", store.users.len());
        for (idx, u) in store.users.iter().enumerate() {
            println!("  [{}] {} ({})", idx + 1, u.username, u.role);
        }
        println!("\n1. Administrator Name (E-Mail) ändern");
        println!("2. Administrator Passwort ändern");
        println!("3. Neuen Administrator hinzufügen");
        println!("4. Beenden");
        print!("Wählen Sie eine Option: ");
        io::stdout().flush()?;

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;

        match choice.trim() {
            "1" => {
                let new_name = prompt_input("Neuer Benutzername (E-Mail): ");
                if !new_name.trim().is_empty() {
                    if let Some(user) = store.users.first_mut() {
                        user.username = new_name.trim().to_string();
                        store.save(store_path)?;
                        println!("ERFOLG: Name erfolgreich geändert.");
                    }
                }
            }
            "2" => {
                print!("Neues Passwort: ");
                io::stdout().flush()?;
                let new_pass = rpassword::read_password()?;
                if new_pass.len() >= 12 {
                    if let Some(user) = store.users.first_mut() {
                        user.password_hash = hash_password(&new_pass);
                        store.save(store_path)?;
                        println!("ERFOLG: Passwort erfolgreich geändert.");
                    }
                } else {
                    println!("[!] Fehler: Das Passwort muss mindestens 12 Zeichen lang sein.");
                }
            }
            "3" => {
                let uname = prompt_input("Benutzername (E-Mail): ");
                print!("Passwort: ");
                io::stdout().flush()?;
                let pass = rpassword::read_password()?;
                if pass.len() >= 12 {
                    let p_hash = hash_password(&pass);
                    store.users.push(AdminUser {
                        username: uname.trim().to_string(),
                        password_hash: p_hash,
                        role: "Admin".to_string(),
                        email_confirmed: true,
                    });
                    store.save(store_path)?;
                    println!("ERFOLG: Neuer Admin angelegt.");
                } else {
                    println!("[!] Fehler: Das Passwort muss mindestens 12 Zeichen lang sein.");
                }
            }
            "4" => break,
            _ => println!("Ungültige Eingabe."),
        }
    }

    Ok(())
}

fn prompt_input(prompt: &str) -> String {
    print!("{}", prompt);
    let _ = io::stdout().flush();
    let mut buffer = String::new();
    let _ = io::stdin().read_line(&mut buffer);
    buffer.trim().to_string()
}
