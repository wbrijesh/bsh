#[cfg(test)]
mod tests {
    use crate::parser;

    #[test]
    fn test_parse_simple_command() {
        let input = "echo hello";
        let parsed = parser::parse(input).expect("Failed to parse command");
        assert_eq!(parsed.command, "echo");
        assert_eq!(parsed.args, vec!["hello"]);
    }

    #[test]
    fn test_parse_quoted_argument() {
        let input = "echo 'hello world'";
        let parsed = parser::parse(input).expect("Failed to parse command");
        assert_eq!(parsed.command, "echo");
        assert_eq!(parsed.args, vec!["hello world"]);
    }

    #[test]
    fn test_parse_empty_input() {
        let input = "";
        let parsed = parser::parse(input);
        assert!(parsed.is_err(), "Empty input should result in error");
    }
}
