use std::io;

fn main(){
    println!("Enter a word please");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to hear you");
    //let name  = "Hai";
    match name.trim() {
        "Good Bye" => println!("You can't leave"),
        "Hai" => println!("Welcome to the party"),
        _ => println!("What are you doing?")
    }
}