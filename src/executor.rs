use crate::{error::ShellError, parser};
use std::env;
use std::process::{Command, Stdio};

pub fn execute(input: &str) -> Result<(), ShellError> {
    // Use the new parser to get the command and arguments
    let parsed_command = parser::parse(input).map_err(|e| ShellError::ParseError(e.to_string()))?;

    match parsed_command.command.as_str() {
        // Handle built-in commands here
        "cd" => {
            let target_dir = if let Some(dir) = parsed_command.args.get(0) {
                // Expand ~ to home directory
                expand_tilde(dir)
            } else {
                // If no argument, default to home directory
                env::var("HOME").map_err(|e| ShellError::EnvVarError(e.to_string()))?
            };

            std::env::set_current_dir(&target_dir).map_err(ShellError::from)?;
        }
        _ => {
            let mut child = Command::new(&parsed_command.command)
                .args(&parsed_command.args)
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()
                .map_err(|_| ShellError::CommandNotFound(parsed_command.command.clone()))?;

            child.wait().map_err(ShellError::from)?;
        }
    }
    Ok(())
}

// Function to expand tilde (~) to home directory
fn expand_tilde(path: &str) -> String {
    if path.starts_with('~') {
        let home = env::var("HOME").unwrap_or_else(|_| String::from("/"));
        if path == "~" {
            home
        } else {
            format!("{}{}", home, &path[1..]) // Append the rest of the path after ~
        }
    } else {
        path.to_string() // Return the original path if not starting with ~
    }
}
