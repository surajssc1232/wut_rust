use crate::history::CommandEntry;
use regex::Regex;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
use syntect::util::as_24_bit_terminal_escaped;
use syntect::easy::HighlightLines;

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
    syntax_set: SyntaxSet,
    theme_set: ThemeSet,
}

impl GeminiClient {
    pub fn new(api_key: String) -> Self {
        let syntax_set = SyntaxSet::load_defaults_newlines();
        let theme_set = ThemeSet::load_defaults();
        Self {
            client: Client::new(),
            api_key,
            syntax_set,
            theme_set,
        }
    }

    pub async fn analyze_commands(&self, commands: &[CommandEntry]) -> Result<(String, Option<String>), String> {
        let prompt = self.format_prompt(commands);

        let request = GeminiRequest {
            contents: vec![Content {
                parts: vec![Part { text: prompt }],
            }],
        };

        // Gemini model as a variable, I use 2.5-flash-lite in prod and the results are better than 2.0 flash while it's much faster.
        const GEMINI_MODEL: &str = "gemini-2.5-flash-lite-preview-06-17";
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
                GEMINI_MODEL,
                self.api_key
        );

        let response = self
            .client
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

        const BLUE: &str = "\x1b[34m";
        const YELLOW: &str = "\x1b[33m";

        const RED: &str = "\x1b[31m";
        const RESET: &str = "\x1b[0m";

        let mut suggestion = None;
        let suggestion_regex = Regex::new(r"(?i)Did you mean:\s*`?([^`\n]+)`?").unwrap();
        if let Some(caps) = suggestion_regex.captures(&gemini_text) {
            suggestion = Some(caps.get(1).unwrap().as_str().to_string());
            gemini_text = suggestion_regex.replace_all(&gemini_text, "").to_string();
        }

        gemini_text = self.convert_markdown_to_ansi(&gemini_text);

        let analysis_regex = Regex::new(r"(?i)Analysis:").unwrap();
        gemini_text = analysis_regex
            .replace_all(
                &gemini_text,
                &format!(
                    "
{}{}{}",
                    BLUE, "Analysis:", RESET
                ),
            )
            .to_string();

        let next_steps_regex = Regex::new(r"(?i)Next Steps:").unwrap();
        gemini_text = next_steps_regex
            .replace_all(
                &gemini_text,
                &format!(
                    "

{}{}{}",
                    YELLOW, "Next Steps:", RESET
                ),
            )
            .to_string();

        if let Some(ref sugg) = suggestion {
            gemini_text.push_str(&format!(
                "


{}{}{}
{}",
                RED,
                "Did you mean:", RESET,sugg.trim()
            ));
        }

        Ok((gemini_text, suggestion))
    }

    pub async fn query_gemini(&self, query: &str) -> Result<String, String> {
        let prompt = format!("You are a helpful assistant. Please answer the following query:\n\n{}", query);

        let request = GeminiRequest {
            contents: vec![Content {
                parts: vec![Part { text: prompt }],
            }],
        };

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent?key={}",
            self.api_key
        );

        let response = self
            .client
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

        let gemini_text = gemini_response
            .candidates
            .into_iter()
            .next()
            .and_then(|c| c.content.parts.into_iter().next())
            .map(|p| p.text)
            .ok_or_else(|| "No response from Gemini".to_string())?;

        Ok(self.convert_markdown_to_ansi(&gemini_text))
    }

    fn wrap_text(&self, text: &str, max_width: usize, current_indent: usize) -> String {
        let mut wrapped_lines = Vec::new();
        let mut current_line = String::new();
        let effective_width = max_width - current_indent;

        // Regex to strip ANSI escape codes
        let ansi_regex = Regex::new(r"\x1b\[[0-9;]*[a-zA-Z]").unwrap();

        for word in text.split_whitespace() {
            let word_stripped = ansi_regex.replace_all(word, "").to_string();
            let current_line_stripped = ansi_regex.replace_all(&current_line, "").to_string();

            if current_line.is_empty() {
                current_line.push_str(word);
            } else if current_line_stripped.len() + 1 + word_stripped.len() <= effective_width {
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
        result.pop();
        result
    }

    fn find_syntax_for_language(&self, lang: &str) -> &syntect::parsing::SyntaxReference {
        let lang_lower = lang.to_lowercase();
        
        // First try by name
        if let Some(syntax) = self.syntax_set.find_syntax_by_name(&lang_lower) {
            return syntax;
        }
        
        // Then try by extension
        if let Some(syntax) = self.syntax_set.find_syntax_by_extension(&lang_lower) {
            return syntax;
        }
        
        // Try common language mappings
        let mapped_lang = match lang_lower.as_str() {
            "js" | "javascript" | "node" => "JavaScript",
            "ts" | "typescript" => "TypeScript",
            "py" | "python" => "Python",
            "rs" | "rust" => "Rust",
            "go" | "golang" => "Go",
            "cpp" | "c++" | "cxx" => "C++",
            "c" => "C",
            "java" => "Java",
            "kt" | "kotlin" => "Kotlin",
            "cs" | "csharp" | "c#" => "C#",
            "rb" | "ruby" => "Ruby",
            "php" => "PHP",
            "swift" => "Swift",
            "scala" => "Scala",
            "clj" | "clojure" => "Clojure",
            "hs" | "haskell" => "Haskell",
            "lua" => "Lua",
            "perl" | "pl" => "Perl",
            "r" => "R",
            "matlab" | "m" => "MATLAB",
            "sh" | "bash" | "shell" => "Bourne Again Shell (bash)",
            "zsh" => "Bourne Again Shell (bash)", // fallback to bash
            "fish" => "fish",
            "ps1" | "powershell" => "PowerShell",
            "bat" | "batch" => "Batch File",
            "html" | "htm" => "HTML",
            "css" => "CSS",
            "scss" | "sass" => "Sass",
            "less" => "CSS", // fallback to CSS
            "xml" => "XML",
            "json" => "JSON",
            "yaml" | "yml" => "YAML",
            "toml" => "TOML",
            "ini" | "cfg" | "conf" => "INI",
            "dockerfile" | "docker" => "Dockerfile",
            "sql" => "SQL",
            "md" | "markdown" => "Markdown",
            "tex" | "latex" => "LaTeX",
            "vim" => "VimL",
            "make" | "makefile" => "Makefile",
            "cmake" => "CMake",
            "gradle" => "Gradle",
            "dart" => "Dart",
            "elm" => "Elm",
            "erlang" | "erl" => "Erlang",
            "elixir" | "ex" => "Elixir",
            "fsharp" | "fs" | "f#" => "F#",
            "ocaml" | "ml" => "OCaml",
            "nim" => "Nim",
            "crystal" | "cr" => "Crystal",
            "d" => "D",
            "zig" => "Zig",
            "v" | "vlang" => "V",
            "assembly" | "asm" => "Assembly x86_64",
            "diff" | "patch" => "Diff",
            "log" => "Log",
            "text" | "txt" => "Plain Text",
            _ => "",
        };
        
        // Try the mapped language name
        if !mapped_lang.is_empty() {
            if let Some(syntax) = self.syntax_set.find_syntax_by_name(mapped_lang) {
                return syntax;
            }
        }
        
        // Try finding by first line patterns for specific languages
        self.syntax_set.find_syntax_plain_text()
    }

    fn convert_markdown_to_ansi(&self, text: &str) -> String {
        let mut result_lines = Vec::new();
        const RESET: &str = "\x1b[0m";
        const BOLD: &str = "\x1b[1m";
        const ITALIC: &str = "\x1b[3m";
        const CYAN: &str = "\x1b[36m";
        const MAX_LINE_WIDTH: usize = 100;

        let numbered_list_start_regex = Regex::new(r"^(\d+\.\s+)(.*)$").unwrap();
        let next_steps_heading_regex = Regex::new(r"(?i)Next Steps:").unwrap();
        let mut current_list_indent = 0;
        let mut in_next_steps_section = false;

        let mut in_code_block = false;
        let mut code_block_lang = String::new();
        let mut code_block_content = Vec::new();

        for line in text.lines() {
            if line.starts_with("```") {
                if in_code_block {
                    // End of code block
                    in_code_block = false;
                    let code = code_block_content.join("\n");
                    code_block_content.clear();

                    if !code.trim().is_empty() {
                        let syntax = self.find_syntax_for_language(&code_block_lang);
                        let theme = &self.theme_set.themes["base16-ocean.dark"];

                        let mut highlighter = HighlightLines::new(syntax, theme);
                        let highlighted_code = code.lines().map(|line| {
                            let ranges: Vec<(syntect::highlighting::Style, &str)> = highlighter.highlight_line(line, &self.syntax_set).unwrap_or_default();
                            if ranges.is_empty() {
                                line.to_string()
                            } else {
                                as_24_bit_terminal_escaped(&ranges[..], false)
                            }
                        }).collect::<Vec<String>>().join("\n");
                        result_lines.push(self.wrap_text(&highlighted_code, MAX_LINE_WIDTH, 0));
                    }
                } else {
                    // Start of code block
                    in_code_block = true;
                    code_block_lang = line.trim_start_matches("```").trim().to_string();
                    // If no language specified, try to detect from content later
                    if code_block_lang.is_empty() {
                        code_block_lang = "text".to_string();
                    }
                }
                continue;
            }

            if in_code_block {
                code_block_content.push(line.to_string());
                continue;
            }

            let mut processed_line = line.to_string();
            let mut line_indent_for_wrapping = 0;

            if next_steps_heading_regex.is_match(&processed_line) {
                in_next_steps_section = true;
                current_list_indent = 0;
            } else if let Some(caps) = numbered_list_start_regex.captures(&processed_line) {
                let num_part = caps.get(1).unwrap().as_str();
                let text_part = caps.get(2).unwrap().as_str();
                current_list_indent = num_part.len();
                if in_next_steps_section {
                    current_list_indent += 4;
                }
                processed_line = format!("{}{}", num_part, text_part);
                line_indent_for_wrapping = current_list_indent;
            } else if current_list_indent > 0 && !processed_line.trim().is_empty() {
                line_indent_for_wrapping = current_list_indent;
                processed_line = format!(
                    "{:<width$}{}",
                    " ",
                    processed_line,
                    width = line_indent_for_wrapping
                );
            } else {
                current_list_indent = 0;
                in_next_steps_section = false;
                line_indent_for_wrapping = 0;
            }

            let bold_regex = Regex::new(r"\*\*(.*?)\*\*|__(.*?)__").unwrap();
            processed_line = bold_regex
                .replace_all(&processed_line, &format!("{}{}{}", BOLD, "$1$2", RESET))
                .to_string();
            
            let italics_regex = Regex::new(r"\*(.*?)\*|_(.*?)_").unwrap();
            processed_line = italics_regex
                .replace_all(&processed_line, &format!("{}{}{}", ITALIC, "$1$2", RESET))
                .to_string();
            
            // Enhanced inline code highlighting
            let monospace_regex = Regex::new(r"`([^`]+)`").unwrap();
            processed_line = monospace_regex
                .replace_all(&processed_line, &format!("{}{}{}", CYAN, "$1", RESET))
                .to_string();
            
            let heading_regex = Regex::new(r"^#\s*(.*)$").unwrap();
            processed_line = heading_regex
                .replace_all(&processed_line, &format!("\n{}{}{}\n", BOLD, "$1", RESET))
                .to_string();
            
            processed_line =
                self.wrap_text(&processed_line, MAX_LINE_WIDTH, line_indent_for_wrapping);

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
                        cmd.command, cmd.output
                    ));
                }
                prompt.push_str("\n");
            }

            prompt.push_str("--- Command to Analyze ---\n");
            prompt.push_str(&format!(
                "Command: {}\nOutput: {}\n\n",
                last_command.command, last_command.output
            ));
        }

        prompt.push_str(
            "Please provide the following for the last command only:

            A brief analysis of the command and its output.

            Any relevant information or next steps, preferably in a numbered list format.

            If the command appears to be a typo or incorrect, provide a suggestion in a new section titled 'Did you mean:' in the format: `suggested_command`


            Keep your response concise and directly focused on the last command."
        );

        prompt
    }
}
