use std::process::ExitStatus;

pub fn execute(args: &[String]) -> Result<ExitStatus, String> {
    let output = args.join(" ");
    println!("{}", output);
    Ok(ExitStatus::default())
}
