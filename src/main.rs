use std::io;
struct Note{
    title:String,
    content:String,
}
fn main() {
    println!("Enter the password:");
loop{

let mut pas = String::new();

io::stdin().read_line(&mut pas).expect("Failed to read");

let pas = pas.trim();


if pas == "letmein"{

    println!("Access Granted");
    break;
}

else{

    println!("Access Denied");
    println!("Please try again:");
}
}

    let mut notes:Vec<Note> = Vec::new();
    
    loop{
        println!("--- NOTE KEEPER ---");
        println!("1. Add Note");
        println!("2. List Notes");
        println!("3. Exit");
        println!("4. Search for Keyword");
        println!("Enter choice:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");
        let choice = input.trim();
    
    if choice == "1"{
        println!("Enter your Title:");
    
        let mut title_input = String::new();
    
        io::stdin().read_line(&mut title_input).expect("Failed to read");
    
        let title_input = title_input.trim().to_string();

        println!("Enter your Content:");
        let mut content_input = String::new();

        io::stdin().read_line(&mut content_input).expect("Failed to read");

        let content_input = content_input.trim().to_string();

        let note_object = Note{
            title:title_input,
            content: content_input,
        };
    
        println!("Notes saved!");
    
        notes.push(note_object);
    }
    else if choice == "2"{
    
        println!("----The following is the list of items----");
        for note_object in &notes{
            
           println!("{}",note_object.title);
           println!("{}",note_object.content); 
        }
    }
    else if choice == "3"{
    
       println!("Exited successfully");
       break;
    }
    else if choice == "4"{

        println!("Enter the keyword:");

        let mut key = String::new();

        io::stdin().read_line(&mut key).expect("Failed to read");

        let key = key.trim();

        for note_object in &notes{

           if note_object.title.to_lowercase().contains(&key.to_lowercase()){

             println!("{}",note_object.title);
             println!("{}",note_object.content);
           }

        }
    }

    else{
        println!("Invalid error! Please enter a number from the choices provided only.")
    }
}
}
