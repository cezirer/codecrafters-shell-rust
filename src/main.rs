// use std::env;
// use std::fs;
use crate::shell::Shell;
// use std::os::unix::fs::PermissionsExt;

mod builtins;
mod command;
mod shell;
mod test;
fn main() {
    let mut shell = Shell::new();
    shell.run();
}
// echo "/tmp/owl/f\n96" "/tmp/owl/f\35" "/tmp/owl/f'\'94"