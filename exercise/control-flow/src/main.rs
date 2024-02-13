fn main(){
    let name  = "Hai";
    match name {
        "Good Bye" => println!("You can't leave"),
        "Hai" => println!("Welcome to the party"),
        _ => println!("What are you doing?")
    }
}