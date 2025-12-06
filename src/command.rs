use regex::Regex;
#[derive(Debug)]
pub struct Command {
    pub name: String,
    pub args: Vec<String>,
}

impl Command {
    pub fn new(input: &str) -> Command {
        // let parts: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();
        let parts = Command::tokenize_input(input);
        if parts.is_empty() {
            return Command {
                name: String::new(),
                args: Vec::new(),
            };
        }
        let a = parts[1..].to_vec();
        Command {
            name: parts.first().unwrap().clone().to_string(),
            args: parts[1..].to_vec(),
        }
    }
    pub fn tokenize_input(input: &str) -> Vec<String> {
        let mut tokens: Vec<String> = Vec::new();
        let mut is_single_quoted = false;
        let mut is_double_quoted = false;
        let mut is_word = false;
        let mut token = String::new();
        for c in input.chars() {
            match c {
                '\'' => {
                    if is_double_quoted {
                        token.push(c);
                    } else if !is_single_quoted {
                        is_single_quoted = true;
                    } else {
                        is_single_quoted = false;
                    }
                }
                '"' => {
                    if !is_double_quoted {
                        is_double_quoted = true;
                    } else {
                        is_double_quoted = false;
                    }
                }
                ' ' | '\n' => {
                    if is_single_quoted || is_double_quoted {
                        token.push(c);
                    } else if is_word {
                        is_word = false;
                        tokens.push(token.clone());
                        token = String::new();
                    }
                }
                _ => {
                    if !is_word {
                        is_word = true;
                    }
                    token.push(c);
                }
            }
        }
        if !token.is_empty() {
            tokens.push(token);
        }
        tokens
    }
    pub fn _split_command_args_re(input: &str) -> (String, String) {
        let re = Regex::new(r"^(\w+)\s+(.+)$").unwrap();
        if let Some(cap) = re.captures(input) {
            println!("{}", cap.get(1).map_or("", |m| m.as_str()));
            println!("{}", cap.get(2).map_or("", |m| m.as_str()));
            let args_str = cap.get(2).map_or("", |m| m.as_str());
            let re_quotes = Regex::new(r"'(.*?)|\b(\w+)\b'").unwrap();
            if let Some(args_iter) = re_quotes.captures(args_str) {
                println!("{:?}", args_iter);
            }
        }
        ("".to_string(), "".to_string())
    }
}
