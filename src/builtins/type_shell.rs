use crate::builtins::Builtins;
use std::process::ExitStatus;
pub fn execute(args: &[String]) -> Result<ExitStatus, String> {
    let b = Builtins::new();
    for cmd in args {
        println!("cmd: {}", cmd);
        b.check_commands(cmd);
    }
    Ok(ExitStatus::default())
}
