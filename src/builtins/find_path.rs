use crate::builtins::Builtins;
use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
pub fn find_command_in_path(command_name: &str) -> Option<String> {
    // 获取环境变量PATH
    if let Ok(path_var) = env::var("PATH") {
        for path in env::split_paths(&path_var) {
            let full_path = path.join(command_name);
            if full_path.exists() && full_path.is_file() {
                // 考虑文件权限，当前仅考虑unix系统
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
// pub fn check_commands(b: Builtins, cmd: &str) -> Option<BuiltinFn> {
//     if b.commands.contains_key(cmd) {
//         println!("{cmd} is a shell builtin");
//         return b.commands.get(cmd).copied();
//     } else if let Some(command_path) = find_command_in_path(cmd) {
//         println!("{cmd} is {command_path}");
//         return None;
//     } else {
//         println!("{cmd}: command not found");
//         return None;
//     }
// }
