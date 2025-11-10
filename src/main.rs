#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;

fn main() {
    // TODO: Uncomment the code below to pass the first stage
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.split(" ").collect::<Vec<&str>>()[0];
        match input {
            "exit" => exit(0),
            _ => {
                print!("{}: command not found\n", input.trim());
                io::stdout().flush().unwrap();
            }
        }
    }
}
