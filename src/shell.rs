use crate::colours::red;
use crate::{error::ShellError, executor, prompt};
use std::io::{self, Write};

pub struct Shell;

impl Shell {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&mut self) -> Result<(), ShellError> {
        loop {
            let prompt = prompt::generate_prompt();
            print!("{}", prompt);
            io::stdout().flush().map_err(ShellError::from)?;

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .map_err(ShellError::from)?;
            let input = input.trim();

            if input.is_empty() {
                continue;
            }
            if input == "exit" {
                println!("Exiting BSH...");
                break;
            }

            match executor::execute(input) {
                Ok(_) => {}
                Err(err) => eprintln!("{}: {}", red("Error"), err),
            }
        }
        Ok(())
    }
}
