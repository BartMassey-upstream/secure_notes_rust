//! # Secure Notes
//! Vamsi Myla
//!
//! Allow secure creation and reading of notes.

use bcrypt::{hash_bytes, verify};
use clap::{Parser, Subcommand};
use prompted::input;
use std::{fs, process, thread, time};

use serde::{Deserialize, Serialize};

macro_rules! bail {
    ($($arg:tt)*) => {{
        eprintln!($($arg)*);
        process::exit(1);
    }};
}

// Defining a json file
const VM_FILE: &str = "notes.json";

#[derive(Serialize, Deserialize, Debug)]
/// A note with title and content.
struct Note {
    title: String,
    content: String,
}

// function to read the string from json file and load it in the vector.
fn load_notes() -> Vec<Note> {
    // 1. Try to read the file
    let read_result = std::fs::read_to_string(VM_FILE);

    match read_result {
        Ok(json_string) => {
            // 2. The file exists! Now try to parse the JSON text.
            let parse_result = serde_json::from_str(&json_string);

            match parse_result {
                Ok(notes) => {
                    // Success! Return the notes.
                    notes
                }
                Err(error) => {
                    // The file exists, but the JSON is broken/corrupted!
                    eprintln!("Warning: Data file is corrupted. Starting with an empty list.");
                    eprintln!("Details: {}", error);
                    Vec::new() // Return an empty vector so the program doesn't crash
                }
            }
        }
        Err(_) => {
            // The file doesn't exist yet (e.g., the very first time running the app).
            // This is completely normal. Just silently return an empty list.
            Vec::new()
        }
    }
}

//function to save the loaded notes
fn save_notes(notes: &Vec<Note>) {
    // 1. Convert vector to string (this rarely fails, but we still handle it)
    let json_string = serde_json::to_string(notes).expect("Failed to serialize data");

    // 2. The risky part: Writing to the hard drive
    let write_result = std::fs::write(VM_FILE, json_string);

    // 3. Match the result!
    match write_result {
        Ok(_) => {
            // It worked! We use '_' because we don't need the actual Ok value
            // We can just silently succeed.
        }
        Err(error) => {
            // It failed! Print a nice user-friendly error.
            // eprintln! is like println!, but specifically for errors.
            eprintln!("Critical Error: Could not save notes to the hard drive!");
            eprintln!("Technical details: {}", error);
        }
    }
}

fn read_stdin() -> String {
    use std::io::Read;

    let mut stdin = std::io::stdin();
    let mut result = String::new();
    if stdin.read_to_string(&mut result).is_err() {
        bail!("could not read standard input");
    }
    result
}

// to display the list of items entered so far
fn list_items(item: &Vec<Note>) {
    println!("----The following is the list of items----");
    for note_object in item {
        println!("{}", note_object.title);
        println!("{}", note_object.content);
    }
}

/// Prompt for the password and exit if auth fails.
fn password_auth() {
    let secret = match fs::read_to_string("secret.txt") {
        Ok(s) => s,
        Err(error) => bail!("Could not read secret.txt: {}", error),
    };
    let secret = secret.trim();

    for attempts in 0..3 {
        let password = input!("Enter the password: ");
        let password = password.trim();
        if verify(password, secret).unwrap_or(false) {
            println!("Access Granted");
            return;
        } else {
            println!("Access Denied");
        }
        if attempts < 2 {
            println!("Please try again in a couple of seconds:");
            thread::sleep(time::Duration::from_secs(2));
        }
    }
    bail!("password auth failed");
}

#[derive(Parser)]
#[command(name = "Secure Notes")]
#[command(about = "A secure CLI for storing notes", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new note
    Add { title: String },
    /// List all notes
    List,
    /// Edit notes
    Edit {
        /// The title of the note
        title: String,
    },
    /// Search for information in title or content
    Search { query: String },
    /// Set the password (only works on first run)
    Password { password: String },
}

/// Run the secure notes app.
fn main() {
    // 1. Parse the command line arguments immediately
    let cli = Cli::parse();

    // 2. Execute the specific command
    match &cli.command {
        Commands::Add { title } => {
            let mut notes = load_notes();
            password_auth();
            let new_note = Note {
                title: title.clone(),
                content: read_stdin(),
            };
            notes.push(new_note);
            println!(" Note added: {}", title);
            save_notes(&notes);
        }
        Commands::List => {
            let notes = load_notes();
            password_auth();
            if notes.is_empty() {
                println!("No notes found.");
            } else {
                list_items(&notes);
            }
        }

        Commands::Edit { title } => {
            let mut notes = load_notes();
            println!("Searching for note: '{}'...", title);

            let note_option = notes.iter_mut().find(|note| note.title == *title);

            match note_option {
                Some(note) => {
                    password_auth();
                    note.content = read_stdin();
                    println!("Note updated!");
                    save_notes(&notes);
                }
                None => {
                    println!("Error: Note not found.");
                }
            }
        }

        Commands::Search { query } => {
            let notes = load_notes();
            password_auth();
            println!("Searching for: '{}'...\n", query);

            let mut found_any = false;

            for note in notes.iter() {
                let title_lower = note.title.to_lowercase();
                let content_lower = note.content.to_lowercase();
                let query_lower = query.to_lowercase();

                if title_lower.contains(&query_lower) || content_lower.contains(&query_lower) {
                    println!("{}: {}", note.title, note.content);
                    found_any = true;
                }
            }

            if !found_any {
                println!("No notes found containing '{}'.", query);
            }
        }

        Commands::Password { password } => {
            if fs::metadata("secret.txt").is_ok() || fs::metadata(VM_FILE).is_ok() {
                eprintln!("Error: Password has already been set.");
            } else {
                let hashed = hash_bytes(password.as_bytes(), bcrypt::DEFAULT_COST)
                    .expect("Failed to hash password");
                let write_result = fs::write("secret.txt", hashed);
                match write_result {
                    Ok(_) => {
                        println!("Password set successfully.");
                    }
                    Err(error) => {
                        eprintln!("Critical Error: Could not save password!");
                        eprintln!("Technical details: {}", error);
                    }
                }
            }
        }
    }
}
