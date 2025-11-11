use std::env;
use std::fs;
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
fn find_command_in_path(command_name: &str) -> Option<String> {
    // 获取环境变量PATH
    if let Ok(path_var) = env::var("PATH") {
        for path in env::split_paths(&path_var) {
            let full_path = path.join(command_name);
            if full_path.exists() && full_path.is_file() {
                if let Ok(metadata) = fs::metadata(&full_path) {
                    let permissions = metadata.permissions();
                    if permissions.mode() & 0o111 != 0 {
                        return Some(full_path.to_string_lossy().into_owned());
                    }
                }
            }
        }
    }
    None
}
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
                    if let Some(args_path) = find_command_in_path(args) {
                        println!("{args} is {args_path}");
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
