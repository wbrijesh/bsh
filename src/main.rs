#![allow(unused)]

mod colours;
mod error;
mod executor;
mod parser;
mod shell;
mod tests;
mod utils;

fn main() -> Result<(), error::ShellError> {
    let mut shell = shell::Shell::new();
    shell.run()?;
    Ok(())
}
