use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::history::CommandEntry;
use regex::Regex;

#[derive(Serialize)]
struct GeminiRequest {
    contents: Vec<Content>,
}

#[derive(Serialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Serialize)]
struct Part {
    text: String,
}

#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
}

#[derive(Deserialize)]
struct Candidate {
    content: ResponseContent,
}

#[derive(Deserialize)]
struct ResponseContent {
    parts: Vec<ResponsePart>,
}

#[derive(Deserialize)]
struct ResponsePart {
    text: String,
}

pub struct GeminiClient {
    client: Client,
    api_key: String,
}

impl GeminiClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    pub async fn analyze_commands(&self, commands: &[CommandEntry]) -> Result<String, String> {
        let prompt = self.format_prompt(commands);
        
        let request = GeminiRequest {
            contents: vec![Content {
                parts: vec![Part { text: prompt }],
            }],
        };

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent?key={}",
            self.api_key
        );

        let response = self.client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("API error: {}", error_text));
        }

        let gemini_response: GeminiResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        let mut gemini_text = gemini_response
            .candidates
            .into_iter()
            .next()
            .and_then(|c| c.content.parts.into_iter().next())
            .map(|p| p.text)
            .ok_or_else(|| "No response from Gemini".to_string())?;

        gemini_text = self.convert_markdown_to_ansi(&gemini_text);

        const BLUE: &str = "\x1b[34m";
        const YELLOW: &str = "\x1b[33m";
        const GREEN: &str = "\x1b[32m";
        const RESET: &str = "\x1b[0m";

        let mut suggestion = None;
        let suggestion_regex = Regex::new(r"Suggestion: `(.*?)`").unwrap();
        if let Some(caps) = suggestion_regex.captures(&gemini_text) {
            suggestion = Some(caps.get(1).unwrap().as_str().to_string());
            gemini_text = suggestion_regex.replace_all(&gemini_text, "").to_string();
        }

        gemini_text = gemini_text.replace("Analysis:", &format!("{}{}{}", BLUE, "Analysis:", RESET));

        gemini_text = gemini_text.replace("Next Steps:", &format!("{}{}{}", YELLOW, "Next Steps:", RESET));

        if let Some(sugg) = suggestion {
            gemini_text.push_str(&format!("\n{}{}{}{}\n", GREEN, "Suggestion:", RESET, format!(" `{}`", sugg)));
        }

        Ok(gemini_text)
    }

    fn wrap_text(&self, text: &str, max_width: usize, current_indent: usize) -> String {
        let mut wrapped_lines = Vec::new();
        let mut current_line = String::new();
        let effective_width = max_width - current_indent;

        for word in text.split_whitespace() {
            if current_line.is_empty() {
                current_line.push_str(word);
            } else if current_line.len() + 1 + word.len() <= effective_width {
                current_line.push(' ');
                current_line.push_str(word);
            } else {
                wrapped_lines.push(current_line);
                current_line = String::from(word);
            }
        }
        wrapped_lines.push(current_line);

        let mut result = String::new();
        for (i, line) in wrapped_lines.into_iter().enumerate() {
            if i > 0 {
                result.push_str(&format!("{:indent$}", " ", indent = current_indent));
            }
            result.push_str(&line);
            result.push('\n');
        }
        result.pop(); // Remove last newline
        result
    }

    fn convert_markdown_to_ansi(&self, text: &str) -> String {
        let mut result_lines = Vec::new();
        const RESET: &str = "\x1b[0m";
        const BOLD: &str = "\x1b[1m";
        const ITALIC: &str = "\x1b[3m";
        const CYAN: &str = "\x1b[36m";
        const MAX_LINE_WIDTH: usize = 100; // Define maximum line width

        let numbered_list_start_regex = Regex::new(r"^(\d+\.\s+)(.*)$").unwrap();
        let mut current_list_indent = 0;

        for line in text.lines() {
            let mut processed_line = line.to_string();

            if let Some(caps) = numbered_list_start_regex.captures(&processed_line) {
                // New list item
                let num_part = caps.get(1).unwrap().as_str();
                let text_part = caps.get(2).unwrap().as_str();
                current_list_indent = num_part.len();
                processed_line = format!("{}{}", num_part, text_part);
            } else if current_list_indent > 0 && !processed_line.trim().is_empty() {
                // Continuation of a list item
                processed_line = format!("{:indent$}{}", " ", processed_line, indent = current_list_indent);
            } else {
                // Not a list item, reset indent
                current_list_indent = 0;
            }

            // Apply other Markdown formatting to the processed line
            // Bold (**text** or __text__)
            let bold_regex = Regex::new(r"\*\*(.*?)\*\*|__(.*?)__").unwrap();
            processed_line = bold_regex.replace_all(&processed_line, &format!("{}{}{}", BOLD, "$1$2", RESET)).to_string();

            // Italics (*text* or _text_)
            let italics_regex = Regex::new(r"\*(.*?)\*|_(.*?)").unwrap();
            processed_line = italics_regex.replace_all(&processed_line, &format!("{}{}{}", ITALIC, "$1$2", RESET)).to_string();

            // Monospace (`text`)
            let monospace_regex = Regex::new(r"`(.*?)`").unwrap();
            processed_line = monospace_regex.replace_all(&processed_line, &format!("{}{}{}", CYAN, "$1", RESET)).to_string();

            // Headings (# Heading)
            let heading_regex = Regex::new(r"^#\s*(.*)$").unwrap();
            processed_line = heading_regex.replace_all(&processed_line, &format!("\n{}{}{}\n", BOLD, "$1", RESET)).to_string();

            // Apply line wrapping after all other formatting
            processed_line = self.wrap_text(&processed_line, MAX_LINE_WIDTH, current_list_indent);

            result_lines.push(processed_line);
        }

        result_lines.join("\n")
    }

    fn format_prompt(&self, commands: &[CommandEntry]) -> String {
        let mut prompt = String::from(
            "You are a helpful shell command assistant. The user has provided a history of their last few commands. \
            Use the full history for context, but focus your analysis and suggestions *only* on the most recent command.\n\n"
        );

        if let Some((last_command, context_commands)) = commands.split_last() {
            if !context_commands.is_empty() {
                prompt.push_str("--- Context (previous commands) ---\n");
                for cmd in context_commands {
                    prompt.push_str(&format!(
                        "Command: {}\nOutput: {}\n\n",
                        cmd.command,
                        cmd.output
                    ));
                }
                prompt.push_str("\n");
            }

            prompt.push_str("--- Command to Analyze ---\n");
            prompt.push_str(&format!(
                "Command: {}\nOutput: {}\n\n",
                last_command.command,
                last_command.output
            ));
        }

        prompt.push_str(
            "Please provide the following for the last command only:

            1. A brief analysis of the command and its output.

            2. Any relevant information or next steps, preferably in a numbered list format.

            If the command appears to be a typo or incorrect, provide a suggestion in a new section titled 'Suggestion:' in the format: `suggested_command`


            Keep your response concise and directly focused on the last command."
        );

        prompt
    }
}