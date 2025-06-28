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

        // Convert Markdown to ANSI before applying specific colors
        gemini_text = self.convert_markdown_to_ansi(&gemini_text);

        // ANSI color codes
        const BLUE: &str = "\x1b[34m"; // For Analysis
        const YELLOW: &str = "\x1b[33m"; // For Next Steps
        const RESET: &str = "\x1b[0m";

        // Apply color to "Analysis:"
        gemini_text = gemini_text.replace("Analysis:", &format!("{}{}{}", BLUE, "Analysis:", RESET));

        // Apply color to "Next Steps:"
        gemini_text = gemini_text.replace("Next Steps:", &format!("{}{}{}", YELLOW, "Next Steps:", RESET));

        Ok(gemini_text)
    }

    fn convert_markdown_to_ansi(&self, text: &str) -> String {
        let mut result = text.to_string();
        const RESET: &str = "\x1b[0m";
        const BOLD: &str = "\x1b[1m";
        const ITALIC: &str = "\x1b[3m";
        const CYAN: &str = "\x1b[36m";

        // Bold (**text** or __text__)
        let bold_regex = Regex::new(r"\*\*(.*?)\*\*|__(.*?)__").unwrap();
        result = bold_regex.replace_all(&result, &format!("{}{}{}", BOLD, "$1$2", RESET)).to_string();

        // Italics (*text* or _text_)
        let italics_regex = Regex::new(r"\*(.*?)\*|_(.*?)_").unwrap();
        result = italics_regex.replace_all(&result, &format!("{}{}{}", ITALIC, "$1$2", RESET)).to_string();

        // Monospace (`text`)
        let monospace_regex = Regex::new(r"`(.*?)`").unwrap();
        result = monospace_regex.replace_all(&result, &format!("{}{}{}", CYAN, "$1", RESET)).to_string();

        // Headings (# Heading)
        let heading_regex = Regex::new(r"^#\s*(.*)$").unwrap();
        result = heading_regex.replace_all(&result, &format!("\n{}{}{}\n", BOLD, "$1", RESET)).to_string();

        // Numbered Lists (1. Item)
        let numbered_list_regex = Regex::new(r"^(\d+\.\s+)(.*)$").unwrap();
        result = numbered_list_regex.replace_all(&result, "  $1$2").to_string();

        result
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
            "Please provide the following for the last command only:\n\
            1. A brief analysis of the command and its output.\n\
            2. Any relevant information or next steps, preferably in a numbered list format.\n\n\
            Keep your response concise and directly focused on the last command."
        );

        prompt
    }
}