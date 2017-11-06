use std::io;

fn main() {
    println!("End of Time Game");
    println!("What is your name?");

    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to real line");

    println!("You guessed: {}", input);
}
