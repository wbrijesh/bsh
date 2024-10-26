#![allow(unused)]

mod error;
mod executor;
mod parser;
mod shell;
mod utils;

fn main() -> Result<(), error::ShellError> {
    let mut shell = shell::Shell::new();
    shell.run()?;
    Ok(())
}
