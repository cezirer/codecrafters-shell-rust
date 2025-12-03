use std::env;
use std::path::Path;
use std::process::ExitStatus;

pub fn execute(args: &[String]) -> Result<ExitStatus, String> {
    let path_arg = args.get(0).ok_or("args error".to_string())?;
    if path_arg.contains("~") {
        let home_path = env::home_dir().ok_or("failed to get home path".to_string())?;
        env::set_current_dir(home_path).map_err(|e| e.to_string())?;
        return Ok(ExitStatus::default());
    }
    let path = Path::new(path_arg);
    env::set_current_dir(path).map_err(|e| {
        let e_str = e.to_string();
        if e_str.contains("No such file or directory") {
            return format!("cd: {path_arg}: No such file or directory");
        }
        e_str
    })?;
    let _ = env::current_dir();
    Ok(ExitStatus::default())
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn test() {
        let _ = execute(&[r"~".to_string()]);
    }
    #[test]
    fn test_no_such_output() {
        let res = execute(&["/aaa".to_string()]);
        println!("{:?}", res);
    }
}
