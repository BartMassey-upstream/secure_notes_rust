//! # Secure Notes
//! Vamsi Myla
//!
//! Allow secure creation and reading of notes.

use bcrypt::verify;
use clap::{Parser,Subcommand};
use std::{fs, io, process, thread, time};

use serde::{Deserialize, Serialize};

extern crate anyhow;
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
    
    match fs::read_to_string(VM_FILE) {  
        Ok(content) => {
            let notes: Vec<Note> = serde_json::from_str(&content).unwrap_or_else(|_| {
                println!("⚠️ Warning: Database corrupted. Starting fresh.");
                Vec::new()
            });
            notes
        }
        Err(_) => {
            Vec::new()
        }
    }
}



//function to save the loaded notes
fn save_notes(notes: &Vec<Note>) {
    let json_content = serde_json::to_string_pretty(notes).expect("Failed to serialize data");
        
    fs::write(VM_FILE, json_content).expect("Failed to write to database");
    
    println!("💾 Database updated.");
}


// to display the list of items entered so far
fn list_items(item: &Vec<Note>) {
    println!("----The following is the list of items----");
    for note_object in item {
        println!("{}", note_object.title);
        println!("{}", note_object.content);
    }
}

/// Get and check the password.
fn password_auth() -> Result<(), anyhow::Error> {
    // Created a file to store password instead of hardcoding (change -> done)

    let secret = fs::read_to_string("secret.txt")?;
    let secret = secret.trim();

    println!("Enter the password:");

    // XXX Limited number of retries, then return failure.(done)
    for attempts in 0..3 {
        let mut password = String::new();
        io::stdin()
            .read_line(&mut password)
            .expect("Failed to read");
        let password = password.trim();
        // XXX Replace hardcoded password! (done)
        // Implemented bcrypt hashing to make the password secure.
        if verify(&password, &secret).unwrap_or(false) {
            println!("Access Granted");
            return Ok(());
        } else {
            println!("Access Denied");
        }
        if attempts < 2 {
            println!("Please try again in a couple of seconds:");
             // XXX Sleep for a second or so to reduce attempts per second.(done)
            thread::sleep(time::Duration::from_secs(2));
            
        }
       
    }
    anyhow::bail!("password auth failed")
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
    Add {
        /// The title of the note
        title: String,
        /// The content of the note
        content: String,
    },
    /// List all notes
    List,
}


/// Run the secure notes app.
fn main() {
    // 1. Parse the command line arguments immediately
    let cli = Cli::parse();

    // 2. Security Check (Same as before)
    if password_auth().is_err() {
        process::exit(1);
    }

    // 3. Initialize Memory
    let mut notes: Vec<Note> = load_notes();

    // 4. Execute the specific command
    match &cli.command {
        Commands::Add { title, content } => {
            // CRITICAL CHANGE: We do NOT call 'add_note()' function here.
            // Why? Because 'add_note()' asks for user input (stdin).
            // We already HAVE the input in 'title' and 'content' variables!
            let new_note = Note {
                title: title.clone(),
                content: content.clone(),
            };
            notes.push(new_note);
            println!(" Note added: {}", title);
            save_notes(&notes);
        }
        Commands::List => {
            // We can reuse this function because it just reads data
            if notes.is_empty() {
                println!("No notes found.");
            } else {
                list_items(&notes);
            }
        }
    }
}
