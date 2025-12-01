use std::env;
use std::process::ExitStatus;

pub fn execute(args: &[String]) -> Result<ExitStatus, String> {
    let path = env::current_dir().unwrap();
    println!("{}", path.to_str().unwrap());
    Ok(ExitStatus::default())
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn basic_test() {
        execute(&[]);
    }
}
