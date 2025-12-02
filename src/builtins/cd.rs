use std::env;
use std::path::Path;
use std::process::ExitStatus;
pub fn execute(args: &[String]) -> Result<ExitStatus, String> {
    let path_arg = args.get(0).ok_or("args error".to_string())?;
    let path = Path::new(path_arg);
    env::set_current_dir(path).map_err(|e| {
        let e_str = e.to_string();
        if e_str.contains("No such file or directory") {
            return format!("cd: {path_arg}: No such file or directory");
        }
        e_str
    })?;
    // println!("{:?}", env::current_dir());
    Ok(ExitStatus::default())
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn test() {
        let _ = execute(&[r"..".to_string()]);
    }
    #[test]
    fn test_no_such_output() {
        let res = execute(&["/aaa".to_string()]);
        println!("{:?}", res);
    }
}
