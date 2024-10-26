use crate::colours::red;
use crate::{error::ShellError, executor};
use std::io::{self, Write};

pub struct Shell {
    prompt: String,
}

impl Shell {
    pub fn new() -> Self {
        Self {
            prompt: "bsh> ".to_string(),
        }
    }

    pub fn run(&mut self) -> Result<(), ShellError> {
        loop {
            print!("{}", self.prompt);
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
