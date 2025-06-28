use std::process::Command;

pub fn get_pane_content() -> String {
    let output = Command::new("tmux")
        .args(["capture-pane", "-p", "-S", "-"])
        .output();
        
    match output {
        Ok(output) if output.status.success() => {
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        }
        _ => {
            // If tmux is not available or command fails, return empty string
            // This will be handled gracefully by the calling code
            String::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::shell_detection::get_shell_info;

    #[test]
    fn test_get_shell_info() {
        let (shell_name, shell_path) = get_shell_info();
        assert!(shell_name.len() > 0);
        assert!(shell_path.len() > 0);
        assert!(shell_name != "error");
    }
}
