use crate::builtins::find_path;
use crate::builtins::Builtins;
use crate::command::Command;
use std::io::{self, Write};
use std::process::{Command as StdCommand, ExitStatus};
pub struct Shell {
    builtins: Builtins,
}

impl Shell {
    pub fn new() -> Self {
        Shell {
            builtins: Builtins::new(),
        }
    }
    pub fn run(&mut self) {
        loop {
            self.print_prompt();
            if let Some(input) = self.read_input() {
                if input.trim().is_empty() {
                    continue;
                }
                self.execute_command(input.as_str());
            }
        }
    }
    fn print_prompt(&self) {
        print!("$ ");
        io::stdout().flush().unwrap();
    }
    fn read_input(&self) -> Option<String> {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                println!();
                std::process::exit(0)
            }
            Ok(_) => Some(input),
            Err(_) => None,
        }
    }
    fn execute_command(&self, input: &str) {
        let command = Command::new(input);
        if command.name.is_empty() {
            return;
        }
        if self.builtins.contains(&command.name) {
            self.execute_builtin(&command);
        } else {
            self.execute_external(&command);
        }
    }
    fn execute_builtin(&self, command: &Command) {
        if let Some(builtin_fn) = self.builtins.get(&command.name) {
            match builtin_fn(&command.args) {
                Ok(_) => {}
                Err(e) => eprintln!("{}", e),
            }
        }
    }
    fn execute_external(&self, command: &Command) {
        if let Some(path) = self.find_external(command) {
            let mut args_all = vec![command.name.clone()];
            args_all.extend(command.args.clone().into_iter());
            StdCommand::new(path).args(args_all);
        }
    }
    fn find_external(&self, command: &Command) -> Option<String> {
        find_path::find_command_in_path(&command.name)
    }
}
