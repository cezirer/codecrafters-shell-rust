// pub mod cd;
pub mod echo;
pub mod exit;
pub mod find_path;
pub mod type_shell;
use std::{collections::HashMap, process::ExitStatus};
pub type BuiltinFn = fn(&[String]) -> Result<ExitStatus, String>;

pub struct Builtins {
    pub commands: HashMap<String, BuiltinFn>,
}

impl Builtins {
    pub fn new() -> Self {
        let mut commands = HashMap::new();

        // commands.insert("cd", cd::execute as BuiltinFn);
        commands.insert("echo".to_string(), echo::execute as BuiltinFn);
        commands.insert("exit".to_string(), exit::execute as BuiltinFn);
        commands.insert("type".to_string(), type_shell::execute as BuiltinFn);
        // commands.insert("type".to_string(), type_shell_builtin::execute as BuiltinFn);

        Builtins { commands }
    }
    pub fn get(&self, name: &str) -> Option<BuiltinFn> {
        return self.commands.get(name).copied();
    }
    fn find_command_in_path(command_name: &str) -> Option<String> {
        // 获取环境变量PATH
        find_path::find_command_in_path(command_name)
    }
    pub fn contains(&self, name: &str) -> bool {
        self.commands.contains_key(name)
    }
    pub fn check_commands(&self, cmd: &str) -> Option<BuiltinFn> {
        if self.commands.contains_key(cmd) {
            println!("{cmd} is a shell builtin");
            return self.commands.get(cmd).copied();
        } else if let Some(command_path) = Builtins::find_command_in_path(cmd) {
            println!("{cmd} is {command_path}");
            return None;
        } else {
            println!("{cmd}: not found");
            return None;
        }
    }
}
