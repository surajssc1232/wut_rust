use crate::shell_detection::detect_current_shell;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommandEntry {
    pub command: String,
    pub output: String,
}

pub struct HistoryManager;

impl HistoryManager {
    pub fn new() -> Result<Self, String> {
        Ok(HistoryManager {})
    }

    pub fn get_last_commands(&self, count: usize) -> Result<Vec<CommandEntry>, String> {
        // Try to find and read history files directly  
        let history_files = self.find_history_files();
        
        for file_path in history_files {
            match self.read_history_file(&file_path, count) {
                Ok(commands) => {
                    if !commands.is_empty() {
                        return Ok(commands);
                    }
                }
                Err(_) => {
                    // Continue to next file if this one fails
                }
            }
        }
        
        // If we're in bash and no history found, provide helpful message
        if detect_current_shell() == "bash" {
            return Err("Could not read bash history. Bash doesn't write to history file immediately. \
                       To fix this, add 'PROMPT_COMMAND=\"history -a\"' to your ~/.bashrc to write history after each command.".to_string());
        }
        
        Err("Could not read any history files".to_string())
    }

    fn find_history_files(&self) -> Vec<String> {
        let mut files = Vec::new();
        
        // Check HISTFILE environment variable first (usually set by current shell)
        if let Ok(histfile) = env::var("HISTFILE") {
            files.push(histfile);
        }
        
        // Detect current shell and prioritize its history file
        let current_shell = detect_current_shell();
        if let Ok(home) = env::var("HOME") {
            match current_shell.as_str() {
                "zsh" => {
                    files.push(format!("{}/.zsh_history", home));
                    // Fallback to other shells
                    files.push(format!("{}/.bash_history", home));
                    files.push(format!("{}/.history", home));
                    files.push(format!("{}/.local/share/fish/fish_history", home));
                }
                "bash" => {
                    files.push(format!("{}/.bash_history", home));
                    files.push(format!("{}/.history", home));
                    // Fallback to other shells
                    files.push(format!("{}/.zsh_history", home));
                    files.push(format!("{}/.local/share/fish/fish_history", home));
                }
                "fish" => {
                    files.push(format!("{}/.local/share/fish/fish_history", home));
                    // Fallback to other shells
                    files.push(format!("{}/.bash_history", home));
                    files.push(format!("{}/.zsh_history", home));
                    files.push(format!("{}/.history", home));
                }
                _ => {
                    // Unknown shell, try all common locations
                    files.push(format!("{}/.bash_history", home));
                    files.push(format!("{}/.zsh_history", home));
                    files.push(format!("{}/.history", home));
                    files.push(format!("{}/.local/share/fish/fish_history", home));
                }
            }
        }
        
        // Remove duplicates while preserving order
        let mut seen = std::collections::HashSet::new();
        let unique_files: Vec<String> = files.into_iter()
            .filter(|f| seen.insert(f.clone()))
            .collect();
        
                // Filter to only existing files
        unique_files.into_iter().filter(|f| Path::new(f).exists()).collect()
    }

    fn read_history_file(&self, file_path: &str, count: usize) -> Result<Vec<CommandEntry>, String> {
        // Read as bytes first to handle potential UTF-8 issues
        let bytes = fs::read(file_path)
            .map_err(|e| format!("Failed to read {}: {}", file_path, e))?;
            
        // Convert to string, replacing invalid UTF-8 with replacement characters
        let content = String::from_utf8_lossy(&bytes).to_string();
            
        // Determine file format based on content
        if content.contains(": ") && content.contains(";") {
            // Zsh format: ": timestamp:duration;command"
            self.parse_zsh_history(&content, count)
        } else if content.contains("- cmd: ") {
            // Fish format
            self.parse_fish_history(&content, count)
        } else {
            // Bash format (simple line-by-line)
            self.parse_bash_history(&content, count)
        }
    }

    fn parse_zsh_history(&self, content: &str, count: usize) -> Result<Vec<CommandEntry>, String> {
        let mut commands = Vec::new();
        
        for line in content.lines().rev() {
            if line.starts_with(": ") {
                // Format: ": 1751143244:0;asdojadjas"
                if let Some(semicolon_pos) = line.find(';') {
                    let command = line[semicolon_pos + 1..].trim();
                    
                    if !command.is_empty() && command != "huh" && command != "wut" && !command.starts_with("history") {
                        commands.push(CommandEntry {
                            command: command.to_string(),
                            output: String::new(),
                        });
                        
                        if commands.len() >= count {
                            break;
                        }
                    }
                }
            }
        }
        
        // Reverse to get chronological order
        commands.reverse();
        Ok(commands)
    }

    fn parse_bash_history(&self, content: &str, count: usize) -> Result<Vec<CommandEntry>, String> {
        let mut commands = Vec::new();
        
        for line in content.lines().rev() {
            let command = line.trim();
            if !command.is_empty() && command != "huh" && command != "wut" && !command.starts_with("history") {
                commands.push(CommandEntry {
                    command: command.to_string(),
                    output: String::new(),
                });
                
                if commands.len() >= count {
                    break;
                }
            }
        }
        
        commands.reverse();
        Ok(commands)
    }

    fn parse_fish_history(&self, content: &str, count: usize) -> Result<Vec<CommandEntry>, String> {
        let mut commands = Vec::new();
        
        for line in content.lines().rev() {
            if line.starts_with("- cmd: ") {
                let command = line[7..].trim(); // Remove "- cmd: " prefix
                if !command.is_empty() && command != "huh" && command != "wut" && !command.starts_with("history") {
                    commands.push(CommandEntry {
                        command: command.to_string(),
                        output: String::new(),
                    });
                    
                    if commands.len() >= count {
                        break;
                    }
                }
            }
        }
        
        commands.reverse();
        Ok(commands)
    }
}




