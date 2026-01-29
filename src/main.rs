use std::io;
struct Note {
    title: String,
    content: String,
}

fn first_choice(book: &mut Vec<Note>) {
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

fn list_items(item: &Vec<Note>) {
    println!("----The following is the list of items----");
    for note_object in item {
        println!("{}", note_object.title);
        println!("{}", note_object.content);
    }
}

fn search(sch: &Vec<Note>) {
    println!("Enter the keyword:");

    let mut key = String::new();

    io::stdin().read_line(&mut key).expect("Failed to read");

    let key = key.trim();

    for note_object in sch {
        if note_object
            .title
            .to_lowercase()
            .contains(&key.to_lowercase())
        {
            println!("{}", note_object.title);
            println!("{}", note_object.content);
        }
    }
}

fn pass_authentication() {
    println!("Enter the password:");
    loop {
        let mut pas = String::new();

        io::stdin().read_line(&mut pas).expect("Failed to read");

        let pas = pas.trim();

        if pas == "letmein" {
            println!("Access Granted");
            break;
        } else {
            println!("Access Denied");
            println!("Please try again:");
        }
    }
}

fn menu() {
    println!("--- NOTE KEEPER ---");
    println!("1. Add Note");
    println!("2. List Notes");
    println!("3. Exit");
    println!("4. Search for Keyword");
    println!("Enter choice:");
}

fn exit() {
    println!("Exited successfully");
}
fn main() {
    pass_authentication();

    let mut notes: Vec<Note> = Vec::new();

    loop {
        menu();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");
        let choice = input.trim();

        if choice == "1" {
            first_choice(&mut notes);
        } else if choice == "2" {
            list_items(&notes);
        } else if choice == "3" {
            exit();
            break;
        } else if choice == "4" {
            search(&notes);
        } else {
            println!("Invalid error! Please enter a number from the choices provided only.")
        }
    }
}
