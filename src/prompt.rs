use regex::Regex;
use std::process::Command;

pub fn get_prompt(shell: &str) -> Result<String, String> {
    // Handle error and unknown shell cases
    if shell == "error" || shell == "unknown" {
        // Try to use a generic fallback approach
        return get_generic_prompt();
    }

    let (cmd, args) = match shell {
        "bash" => ("bash", vec!["-c", "echo \"$PS1\""]),
        "zsh" => ("zsh", vec!["-i", "-c", "print -P \"%_$PS1\""]),
        "fish" => ("fish", vec!["-c", "fish_prompt"]),
        _ => return get_generic_prompt(),
    };

    let output = Command::new(cmd)
        .args(&args)
        .output()
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    if !output.status.success() {
        // If the shell command fails, try the generic approach
        return get_generic_prompt();
    }

    let raw = String::from_utf8_lossy(&output.stdout);
    let prompt = raw.lines().last().unwrap_or("").trim().to_string();
    
    if prompt.is_empty() {
        return get_generic_prompt();
    }
    
    Ok(prompt)
}

fn get_generic_prompt() -> Result<String, String> {
    // Try to get username and hostname for a generic prompt
    use std::env;
    
    let user = env::var("USER").or_else(|_| env::var("USERNAME")).unwrap_or_else(|_| "user".to_string());
    let hostname = env::var("HOSTNAME").unwrap_or_else(|_| {
        // Try to get hostname from system
        std::process::Command::new("hostname")
            .output()
            .ok()
            .and_then(|output| {
                if output.status.success() {
                    Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
                } else {
                    None
                }
            })
            .unwrap_or_else(|| "localhost".to_string())
    });
    
    // Create a generic prompt pattern that should work for most shells
    Ok(format!("{}@{}:", user, hostname))
}

pub fn clean_prompt(prompt: &str) -> String {
    let re_ansi = Regex::new(r"\x1b\[[0-9;]*[a-zA-Z]").unwrap();
    let re_escapes = Regex::new(r"\\\[|\\\]").unwrap();
    let cleaned = re_ansi.replace_all(prompt, "");
    re_escapes.replace_all(&cleaned, "").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_prompt() {
        let input = "\x1b[32m\\[test\\]content\x1b[0m\\[end\\]";
        let expected = "testcontentend";
        let result = clean_prompt(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_clean_prompt_no_escapes() {
        let input = "user@host:~$";
        assert_eq!(clean_prompt(input), input);
    }
}