#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;

fn main() {
    // TODO: Uncomment the code below to pass the first stage
    let _built_in_commands = vec!["echo", "type", "exit"];
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        let mut iter = command.trim().split_whitespace();
        match iter.next() {
            Some("exit") => match iter.next() {
                Some(arg) => match arg {
                    "0" => break,
                    _ => panic!(),
                },
                None => panic!(),
            },
            Some("echo") => println!("{}", iter.collect::<Vec<&str>>().join(" ")),
            Some("type") => {
                if let Some(args) = iter.next() {
                    if _built_in_commands.contains(&args) {
                        println!("{args} is a shell builtin");
                    } else {
                        println!("{args}: not found");
                    }
                }
            }
            Some(cmd) => {
                print!("{}: command not found\n", cmd.trim());
                io::stdout().flush().unwrap();
            }
            None => println!("No command"),
        }
    }
}
