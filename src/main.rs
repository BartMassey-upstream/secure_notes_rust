//! # Secure Notes
//! Vamsi Myla
//!
//! Allow secure creation and reading of notes.


use std::{fs, io, process, thread, time};

extern crate anyhow;

/// A note with title and content.
struct Note {
    title: String,
    content: String,
}

/// Add a note to storage.
fn add_note(book: &mut Vec<Note>) {
    println!("Enter your Title:");
    let mut title_input = String::new();
    io::stdin()
        .read_line(&mut title_input)
        .expect("Failed to read");
    let title_input = title_input.trim().to_string();

    println!("Enter your Content:");
    let mut content_input = String::new();
    io::stdin()
        .read_line(&mut content_input)
        .expect("Failed to read");
    let content_input = content_input.trim().to_string();

    let note_object = Note {
        title: title_input,
        content: content_input,
    };

    println!("Notes saved!");
    book.push(note_object);
}

/// Show title and content of all notes.
// XXX Allow just titles.
fn list_items(item: &Vec<Note>) {
    println!("----The following is the list of items----");
    for note_object in item {
        println!("{}", note_object.title);
        println!("{}", note_object.content);
    }
}

/// Search note titles for matching string.
fn search(notes: &Vec<Note>) {
    println!("Search text:");

    let mut key = String::new();
    io::stdin().read_line(&mut key).expect("Failed to read");
    let key = key.trim().to_lowercase();

    for note_object in notes {
        let is_matching = note_object.title.to_lowercase().contains(&key);

        if is_matching {
            println!("{}", note_object.title);
            println!("{}", note_object.content);
        }
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
        // XXX Replace hardcoded password!
        if password == secret {
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

/// Run the secure notes app.
fn main() {
    if password_auth().is_err() {
        process::exit(1);
    }

    let mut notes: Vec<Note> = Vec::new();

    loop {
        println!("--- NOTE KEEPER ---");
        println!("1. Add Note");
        println!("2. List Notes");
        println!("3. Search for Keyword");
        println!("4. Exit");
        println!("Enter choice:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");
        let choice = input.trim();

        match choice {
            "1" => add_note(&mut notes),
            "2" => list_items(&notes),
            "3" => search(&notes),
            "4" => break,
            _ => println!("Unknown choice."),
        }
    }
}
