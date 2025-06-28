use std::{env, process::Command};

pub fn get_shell_info() -> (String, String) {
    match env::var("SHELL") {
        Ok(shell_path) => {
            let shell_name = if shell_path.contains("zsh") {
                "zsh"
            } else if shell_path.contains("bash") {
                "bash"
            } else if shell_path.contains("fish") {
                "fish"
            } else {
                "unknown"
            };

            (shell_name.to_string(), shell_path.to_string())
        }

        Err(_) => (
            "error".to_string(),
            "could not read SHELL env variable".to_string(),
        ),
    }
}

pub fn get_pane_content() -> String {
    let output = Command::new("tmux")
        .args(["capture-pane", "-p", "-S", "-"])
        .output()
        .expect("Error reading the prompt");

    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

use std::io::{self, Write};

pub fn execute_command(command: &str) {
    let output = Command::new("tmux")
        .args(["send-keys", command, "C-m"])
        .output();

    match output {
        Ok(o) => {
            if !o.status.success() {
                eprintln!("Failed to send command via tmux: {:?}", o.status.code());
                eprintln!("Stdout: {}", String::from_utf8_lossy(&o.stdout));
                eprintln!("Stderr: {}", String::from_utf8_lossy(&o.stderr));
                println!("Command could not be automatically executed. Please press Enter to run it:");
                println!("{}", command);
                io::stdout().flush().unwrap();
            }
        }
        Err(e) => {
            eprintln!("Could not execute tmux command: {}", e);
            println!("Command could not be automatically executed. Please press Enter to run it:");
            println!("{}", command);
            io::stdout().flush().unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_shell_info() {
        let (shell_name, shell_path) = get_shell_info();
        assert!(shell_name.len() > 0);
        assert!(shell_path.len() > 0);
        assert!(shell_name != "error");
    }
}


