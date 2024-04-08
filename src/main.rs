use std::env;
use std::fs::{File, OpenOptions};
use std::path::Path; 
use std::io;
use std::io::Write;

fn main() {
    let file_path = "C:/Users/Ross/OneDrive/Documents/Programs/Rust/Misc/todo/src/list.txt";
    match check_file(file_path) {
        Ok(file) => {
            // File was successfully opened or created
            // Do something with the file here
        }
        Err(err) => {
            // Handle the error
            eprintln!("Error: {}", err);
            // Optionally, you can panic or return early from the main function
            // panic!("Failed to open or create file");
            // return;
        }
    }
        
    println!("Choose Option");
    println!("1 - Add to list");
    println!("2 - Remove from list");
    println!("3 - Mark as completed");
    println!("4 - Show list");

    let args: Vec<String> = env::args().collect();
    
    let choice = args[1].clone();
    let input = args[2].clone();

    if choice =="add"{
        match add(&file_path, input) {
            Ok(()) => println!("File written successfully."),
            Err(err) => eprintln!("Error writing to file: {}", err),
        }
    } else if choice =="2" {
        remove()
    } else if choice =="3" {
        complete()
    } else if choice == "4" {
        list()
    } else {
        println!("Invalid Input");
    }
}

fn check_file(file_path: &str) -> Result<File, std::io::Error> {

    let exists = Path::new(&file_path).exists();

    if !exists {
        println!("File not found! Creating new file...");
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&file_path)?;
        Ok(file)
    } else {
        println!("File found!");
        let file = File::open(&file_path)?;
        Ok(file)
    }
}

fn add(file_path: &str, input: String) -> Result<(), io::Error> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(&file_path)?;

    // Write the input string to the file
    file.write_all(input.as_bytes())?;

    // If necessary, you can return Ok(()) to indicate success
    Ok(())
}

fn remove(){
    println!("removing")
}

fn complete(){
    println!("complete")
}

fn list(){
    println!("listing")
}
