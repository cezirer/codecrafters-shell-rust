#[derive(Debug)]
pub struct Command {
    pub name: String,
    pub args: Vec<String>,
}

impl Command {
    pub fn new(input: &str) -> Command {
        let parts: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();
        if parts.is_empty() {
            return Command {
                name: String::new(),
                args: Vec::new(),
            };
        }
        Command {
            name: parts.first().unwrap().clone().to_string(),
            args: parts[1..].to_vec(),
        }
    }
}
