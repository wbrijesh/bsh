use crate::colours::{blue, green, yellow};
use std::env;
use std::path::Path;
use std::process::Command;

pub fn generate_prompt() -> String {
    let current_dir = env::current_dir().unwrap_or_else(|_| Path::new("").to_path_buf());
    let current_dir_str = current_dir.to_str().unwrap_or("");

    let home_dir = env::var("HOME").unwrap_or_else(|_| String::from("/"));
    let display_dir = if current_dir_str.starts_with(&home_dir) {
        current_dir_str.replacen(&home_dir, "~", 1)
    } else {
        current_dir_str.to_string()
    };

    let branch_name = if current_dir.join(".git").exists() {
        Command::new("git")
            .args(&["rev-parse", "--abbrev-ref", "HEAD"])
            .output()
            .ok()
            .and_then(|output| {
                if output.status.success() {
                    String::from_utf8(output.stdout)
                        .ok()
                        .map(|s| s.trim().to_string())
                } else {
                    None
                }
            })
            .unwrap_or_else(|| "".to_string())
    } else {
        "".to_string()
    };

    format!(
        "{} {} {} ",
        blue(&display_dir),
        green(&branch_name),
        yellow("❯")
    )
}
