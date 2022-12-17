use std::io;

// get name function
fn get_name() -> String {
    println!("What's your name?");
    let mut name = String::new();

    // getting name from user
    io::stdin().read_line(&mut name).expect("Failed to read line");

    return name;
}

// function print "Hello, name"
fn hello_user(name: String) {
    println!("Hello, {}!", name.trim());
}

// main function
fn main() {
    println!("Hello, world!");
    println!("Hello, world, again!"); 

    let name: String = get_name();

    hello_user(name);
}
