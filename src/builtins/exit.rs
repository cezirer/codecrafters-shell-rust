use std::process;
use std::process::ExitStatus;

pub fn execute(args: &[String]) -> Result<ExitStatus, String> {
    let code = if let Some(arg) = args.get(0) {
        arg.parse::<i32>().unwrap()
    } else {
        0
    };
    process::exit(code);
}
