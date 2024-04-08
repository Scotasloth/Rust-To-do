use std::env;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::Path; 

fn main() {

    match check_file() {
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
    println!("{:?}", args);
    let input = args[1].clone();

    if input =="1"{
        add()
    } else if input =="2" {
        remove()
    } else if input =="3" {
        complete()
    } else if input == "4" {
        list()
    } else {
        println!("Invalid Input");
    }
}

fn check_file() -> Result<File, std::io::Error> {
    let file_path = "C:/Users/Ross/OneDrive/Documents/Programs/Rust/Misc/todo/src/list.txt";

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



fn add() {
    println!("adding")
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
