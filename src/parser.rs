use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct ParsedCommand {
    pub command: String,
    pub args: Vec<String>,
}

pub fn parse(input: &str) -> Result<ParsedCommand, &'static str> {
    if input.trim().is_empty() {
        return Err("Empty input should result in error");
    }

    let mut command = String::new();
    let mut args = Vec::new();
    let mut current_arg = String::new();
    let mut in_single_quote = false;
    let mut in_double_quote = false;
    let mut escape_next = false;

    let chars = input.chars().collect::<Vec<_>>();

    for c in chars {
        if escape_next {
            match c {
                'r' => current_arg.push('\r'),
                'n' => current_arg.push('\n'),
                't' => current_arg.push('\t'),
                _ => current_arg.push(c),
            }
            escape_next = false;
            continue;
        }

        match c {
            '\\' => {
                escape_next = true; // Escape the next character
            }
            '\'' if !in_double_quote => {
                in_single_quote = !in_single_quote; // Toggle single quote state
            }
            '"' if !in_single_quote => {
                in_double_quote = !in_double_quote; // Toggle double quote state
            }
            ' ' if !in_single_quote && !in_double_quote => {
                // If we encounter a space and we're not in quotes, we finalize the current argument
                if !current_arg.is_empty() {
                    if command.is_empty() {
                        command = current_arg.clone();
                    } else {
                        args.push(current_arg.clone());
                    }
                    current_arg.clear();
                }
            }
            _ => {
                current_arg.push(c); // Add character to the current argument
            }
        }
    }

    if in_single_quote || in_double_quote {
        return Err("Unclosed quote in input");
    }

    // Finalize the last argument
    if !current_arg.is_empty() {
        if command.is_empty() {
            command = current_arg.clone();
        } else {
            args.push(current_arg);
        }
    }

    Ok(ParsedCommand { command, args })
}
