use regex::Regex;
use std::process::Command;

pub fn get_prompt(shell: &str) -> Result<String, String> {
    let (cmd, args) = match shell {
        "bash" => ("bash", vec!["-c", "echo \"$PS1\""]),
        "zsh" => ("zsh", vec!["-i", "-c", "print -P \"%_$PS1\""]),
        "fish" => ("fish", vec!["-c", "fish_prompt"]),
        _ => return Err(format!("Unsupported shell: {}", shell)),
    };

    let output = Command::new(cmd)
        .args(&args)
        .output()
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    if !output.status.success() {
        return Err(format!("Command failed with exit code: {:?}", output.status.code()));
    }

    let raw = String::from_utf8_lossy(&output.stdout);
    Ok(raw.lines().last().unwrap_or("").trim().to_string())
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
        let input = r"\x1b[32m\[test\]content\x1b[0m\[end\]";
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