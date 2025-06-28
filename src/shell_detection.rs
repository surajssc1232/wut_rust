use std::env;

pub fn detect_current_shell() -> String {
    // Try to detect current shell from various sources
    
    // Check if we're running under a specific shell based on environment variables
    if env::var("ZSH_VERSION").is_ok() {
        return "zsh".to_string();
    }
    
    if env::var("BASH_VERSION").is_ok() {
        return "bash".to_string();
    }
    
    // Check parent process name (this is what shell we're running in)
    if let Ok(output) = std::process::Command::new("ps")
        .args(&["-p", &std::process::id().to_string(), "-o", "ppid="])
        .output()
    {
        if let Ok(ppid) = String::from_utf8_lossy(&output.stdout).trim().parse::<u32>() {
            if let Ok(output) = std::process::Command::new("ps")
                .args(&["-p", &ppid.to_string(), "-o", "comm="])
                .output()
            {
                let parent_name = String::from_utf8_lossy(&output.stdout).trim().to_lowercase();
                if parent_name.contains("zsh") {
                    return "zsh".to_string();
                } else if parent_name.contains("bash") {
                    return "bash".to_string();
                } else if parent_name.contains("fish") {
                    return "fish".to_string();
                }
            }
        }
    }
    
    // Fallback: check SHELL environment variable (default shell, not current)
    if let Ok(shell) = env::var("SHELL") {
        if shell.contains("zsh") {
            return "zsh".to_string();
        } else if shell.contains("bash") {
            return "bash".to_string();
        } else if shell.contains("fish") {
            return "fish".to_string();
        }
    }
    
    // Default fallback
    "unknown".to_string()
}

pub fn get_shell_info() -> (String, String) {
    let shell_name = detect_current_shell();
    let shell_path = env::var("SHELL").unwrap_or_else(|_| "detected".to_string());
    (shell_name, shell_path)
} 