use std::env;

fn main() {

    check_file();

    println!("Choose Option");
    println!("1 - Add to list");
    println!("2 - Remove from list");
    println!("3 - Mark as completed");
    println!("4 - Show list");

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let input = args[1].clone();

    println!("{:?}", input);

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

fn check_file() {
    let dir = r"C:\Users\Ross\OneDrive\Documents\Programs\Rust\Misc\todo>";

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
