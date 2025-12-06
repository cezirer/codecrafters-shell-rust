#[cfg(test)]
mod command_tests {
    use crate::command::Command;
    #[test]
    fn init_command() {
        let c = Command::new("echo 'hello' world");
    }

    #[test]
    fn test_tokenize_double_quote() {
        let s = "echo \"hello     world\"\n";
        let rslt = Command::tokenize_input(s);
        assert_eq!(
            vec!["echo".to_string(), "hello     world".to_string(),],
            rslt
        );
        let s = "echo \"hello\"\"world\"\n";
        let rslt = Command::tokenize_input(s);
        assert_eq!(vec!["echo".to_string(), "helloworld".to_string(),], rslt);

        let s = "echo \"hello\" \"world\"\n";
        let rslt = Command::tokenize_input(s);
        assert_eq!(
            vec!["echo".to_string(), "hello".to_string(), "world".to_string()],
            rslt
        );

        let s = "echo \"shell's test\"\n";
        let rslt = Command::tokenize_input(s);
        assert_eq!(vec!["echo".to_string(), "shell's test".to_string(),], rslt);
    }

    #[test]
    fn test_tokenize() {
        let mut s = "echo 'hello ass' 'world'";
        let rslt = Command::tokenize_input(s);
        assert_eq!(
            vec![
                "echo".to_string(),
                "hello ass".to_string(),
                "world".to_string()
            ],
            rslt
        );
        let mut s = "echo 'hello      world'\n";
        let rslt = Command::tokenize_input(s);
        assert_eq!(
            vec!["echo".to_string(), "hello      world".to_string(),],
            rslt
        );
        let mut s = "echo hello    world\n";
        let rslt = Command::tokenize_input(s);
        assert_eq!(
            vec!["echo".to_string(), "hello".to_string(), "world".to_string()],
            rslt
        );
        let mut s = "echo 'hello''world'\n";
        let rslt = Command::tokenize_input(s);
        assert_eq!(vec!["echo".to_string(), "helloworld".to_string()], rslt);
        let mut s = "echo hello''world\n";
        let rslt = Command::tokenize_input(s);
        assert_eq!(vec!["echo".to_string(), "helloworld".to_string()], rslt);
    }
}
