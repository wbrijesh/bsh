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

    #[test]
    fn test_parse_escape_character() {
        let input = "echo hello\\ world";
        let parsed = parser::parse(input).expect("Failed to parse command");
        assert_eq!(parsed.command, "echo");
        assert_eq!(parsed.args, vec!["hello world"]);
    }

    #[test]
    fn test_parse_unclosed_quote() {
        let input = "echo \"hello";
        let parsed = parser::parse(input);
        assert!(parsed.is_err(), "Unclosed quote should result in error");
        assert_eq!(parsed.unwrap_err(), "Unclosed quote in input");
    }

    #[test]
    fn test_parse_carriage_return() {
        let input = "echo hello\rworld";
        let parsed = parser::parse(input).expect("Failed to parse command");
        assert_eq!(parsed.command, "echo");
        assert_eq!(parsed.args, vec!["hello\rworld"]);
    }

    #[test]
    fn test_parse_newline() {
        let input = "echo hello\nworld";
        let parsed = parser::parse(input).expect("Failed to parse command");
        assert_eq!(parsed.command, "echo");
        assert_eq!(parsed.args, vec!["hello\nworld"]);
    }

    #[test]
    fn test_parse_tab() {
        let input = "echo hello\tworld";
        let parsed = parser::parse(input).expect("Failed to parse command");
        assert_eq!(parsed.command, "echo");
        assert_eq!(parsed.args, vec!["hello\tworld"]);
    }
}
