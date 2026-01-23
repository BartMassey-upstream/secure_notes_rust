use std::io;

fn main(){
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

    let mut notes:Vec<String> = Vec::new();
    
    loop{
        println!("--- NOTE KEEPER ---");
        println!("1. Add Note");
        println!("2. List Notes");
        println!("3. Exit");
        println!("Enter choice:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");
        let choice = input.trim();
    
    if choice == "1"{
        println!("Enter a String:");
    
        let mut new_notes = String::new();
    
        io::stdin().read_line(&mut new_notes).expect("Failed to read");
    
        let new_notes = new_notes.trim().to_string();
    
        println!("Notes saved,{new_notes}");
    
        notes.push(new_notes);
    }
    else if choice == "2"{
    
        println!("----The following is the list of items----");
        for new_notes in &notes{
            
           println!("{new_notes}"); 
        }
    }
    else if choice == "3"{
    
       println!("Exited successfully");
       break;
    }
    else{
        println!("Invalid error! Please enter a number from the choices provided only.")
    }
}
}
