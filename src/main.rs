#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;

fn main() {
    // TODO: Uncomment the code below to pass the first stage
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut o_input = String::new();
        io::stdin().read_line(&mut o_input).unwrap();
        let o_input = o_input.splitn(2, " ").collect::<Vec<&str>>();
        let cmd = o_input[0];
        let param = *o_input.get(1).unwrap_or(&&"");
        match cmd {
            "exit" => exit(0),
            "echo" => {
                print!("{}", param);
                io::stdout().flush().unwrap();
            }
            _ => {
                print!("{}: command not found\n", cmd.trim());
                io::stdout().flush().unwrap();
            }
        }
    }
}
