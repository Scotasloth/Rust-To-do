use std::env;
use std::fs::{File, OpenOptions};
use std::path::Path; 
use std::io::{self, BufRead, BufReader, Read, Write};

fn main() {
    let file_path = "C:/Users/Ross/OneDrive/Documents/Programs/Rust/Misc/todo/src/list.txt";
    match check_file(file_path) {
        Ok(file) => {
    
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
        
    let args: Vec<String> = env::args().collect();
    
    let choice = args[1].clone();
    let mut input = String::new(); // Declare input outside the if block

    if args.len() >= 3 {
        input = args[2].clone(); // Update input if condition is true
    }

    if choice =="add"{
        match add(&file_path, input) {
            Ok(()) => println!("File written successfully."),
            Err(err) => eprintln!("Error writing to file: {}", err),
        }
    } else if choice =="remove" {
        remove(&file_path, input)
    } else if choice =="done" {
        complete(&file_path, input)
    } else if choice == "show" {
        match list(&file_path) {
            Ok(()) => println!("Read Successful."),
            Err(err) => eprintln!("Error writing to file: {}", err),
        }
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
    Ok(())
}

fn remove(file_path: &str, input: String){
    println!("removing")
}

fn complete(file_path: &str, input: String){
    println!("complete")
}

fn list(file_path: &str) -> Result<(), io::Error>{
    
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}
