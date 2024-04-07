use std::env;

fn main() {
    println!("Choose Option");
    println!("1 - Add to list");
    println!("2 - Remove from list");
    println!("3 - Mark as completed");
    println!("4 - Show list");

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}

//fn add() {

//}

//fn remove(){

//}

//fn complete(){

//}
