use serde::{Deserialize, Serialize};
use crate::{shell, prompt};

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
        let pane_content = shell::get_pane_content();
        let (shell_name, _) = shell::get_shell_info();

        let prompt_string = prompt::get_prompt(&shell_name).unwrap_or_default();
        let cleaned_prompt = prompt::clean_prompt(&prompt_string);

        if cleaned_prompt.is_empty() {
            return Err("Could not determine shell prompt.".to_string());
        }

        let mut commands = Vec::new();
        let mut end = pane_content.len();
        let mut blocks_found = 0;

        // We look for count + 1 blocks to account for skipping the `huh` command itself.
        while blocks_found < count + 1 {
            if let Some(prompt_pos) = pane_content[..end].rfind(&cleaned_prompt) {
                let block_start = prompt_pos + cleaned_prompt.len();
                let block_end = end;
                let block = &pane_content[block_start..block_end].trim();

                end = prompt_pos;
                blocks_found += 1;

                // The first block found is the `huh` command itself, so we skip it.
                if blocks_found == 1 {
                    continue;
                }

                if !block.is_empty() {
                    let mut lines = block.lines();
                    if let Some(command_line) = lines.next() {
                        let command = command_line.trim().to_string();
                        if !command.is_empty() {
                            let output = lines.collect::<Vec<&str>>().join("\n").trim().to_string();
                            commands.push(CommandEntry { command, output });
                        }
                    }
                }
            } else {
                break;
            }
        }

        Ok(commands)
    }
}




