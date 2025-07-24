use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use dialoguer::{Select, theme::ColorfulTheme};
use console::style;

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub default_model: String,
    pub response_length: String,
    pub temperature: f32,
    pub max_output_tokens: u32,
    pub show_thinking: bool,
    pub auto_save_history: bool,
    pub default_shell: String,
    pub api_timeout: u64,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            default_model: "gemini-2.0-flash".to_string(),
            response_length: "balanced".to_string(),
            temperature: 0.7,
            max_output_tokens: 8192,
            show_thinking: false,
            auto_save_history: true,
            default_shell: "bash".to_string(),
            api_timeout: 30,
        }
    }
}

impl Config {
    pub fn get_response_length_instruction(&self) -> &str {
        match self.response_length.as_str() {
            "brief" => "Keep your response brief and concise. Provide only the essential information without elaborate explanations.",
            "balanced" => "Provide a balanced response with moderate detail. Include key information and brief explanations.",
            "detailed" => "Provide a detailed response with comprehensive explanations. Include relevant context and examples where helpful.",
            "verbose" => "Provide a very detailed and thorough response. Include comprehensive explanations, examples, context, and additional relevant information.",
            _ => "Provide a balanced response with moderate detail."
        }
    }
}

pub struct ConfigManager {
    config_path: PathBuf,
}

impl ConfigManager {
    pub fn new() -> Result<Self, String> {
        let config_dir = dirs::config_dir()
            .ok_or("Unable to determine config directory")?
            .join("huh");
        
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)
                .map_err(|e| format!("Failed to create config directory: {}", e))?;
        }

        let config_path = config_dir.join("config.json");
        
        Ok(ConfigManager { config_path })
    }

    pub fn config_exists(&self) -> bool {
        self.config_path.exists()
    }

    pub fn load_config(&self) -> Result<Config, String> {
        if !self.config_exists() {
            return Ok(Config::default());
        }

        let config_content = fs::read_to_string(&self.config_path)
            .map_err(|e| format!("Failed to read config file: {}", e))?;

        // Try to parse as new format first
        match serde_json::from_str::<Config>(&config_content) {
            Ok(config) => Ok(config),
            Err(_) => {
                // If that fails, try to parse as old format and upgrade
                let old_config: serde_json::Value = serde_json::from_str(&config_content)
                    .map_err(|e| format!("Failed to parse config file: {}", e))?;
                
                let mut new_config = Config::default();
                
                // Migrate old values if they exist
                if let Some(default_model) = old_config.get("default_model").and_then(|v| v.as_str()) {
                    new_config.default_model = default_model.to_string();
                }
                if let Some(response_length) = old_config.get("response_length").and_then(|v| v.as_str()) {
                    new_config.response_length = response_length.to_string();
                }
                
                // Save the upgraded config
                self.save_config(&new_config)?;
                Ok(new_config)
            }
        }
    }

    pub fn save_config(&self, config: &Config) -> Result<(), String> {
        let config_json = serde_json::to_string_pretty(config)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;

        fs::write(&self.config_path, config_json)
            .map_err(|e| format!("Failed to write config file: {}", e))
    }

    pub fn interactive_config_menu(&self) -> Result<Config, String> {
        println!("\\n{}", style("🔧 Configuration Menu").bold().cyan());
        println!("{}", style("Configure your huh settings:").dim());
        println!();

        let current_config = self.load_config()?;
        
        let options = vec![
            "Change Default Model",
            "Change Response Length",
            "Change Temperature",
            "Change Max Output Tokens",
            "Toggle Thinking Process Display",
            "Toggle Auto-Save History",
            "Change Default Shell",
            "Change API Timeout",
            "Show Current Configuration",
            "Exit"
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("What would you like to configure?")
            .items(&options)
            .interact()
            .map_err(|e| format!("Failed to get user selection: {}", e))?;

        match selection {
            0 => self.change_model(),
            1 => self.change_response_length(),
            2 => self.change_temperature(),
            3 => self.change_max_tokens(),
            4 => self.toggle_thinking(),
            5 => self.toggle_auto_save(),
            6 => self.change_shell(),
            7 => self.change_timeout(),
            8 => {
                println!();
                println!("{}", style("Current Configuration:").bold().cyan());
                println!("  Default model: {}", style(&current_config.default_model).cyan());
                println!("  Response length: {}", style(&current_config.response_length).cyan());
                println!("  Temperature: {}", style(&current_config.temperature.to_string()).cyan());
                println!("  Max output tokens: {}", style(&current_config.max_output_tokens.to_string()).cyan());
                println!("  Show thinking: {}", style(&current_config.show_thinking.to_string()).cyan());
                println!("  Auto-save history: {}", style(&current_config.auto_save_history.to_string()).cyan());
                println!("  Default shell: {}", style(&current_config.default_shell).cyan());
                println!("  API timeout: {} seconds", style(&current_config.api_timeout.to_string()).cyan());
                println!();
                Ok(current_config)
            }
            9 => {
                println!("Configuration unchanged.");
                Ok(current_config)
            }
            _ => Ok(current_config)
        }
    }

    pub fn change_model(&self) -> Result<Config, String> {
        println!("\\n{}", style("🔧 Change Default Model").bold().cyan());
        println!("{}", style("Select your new default Gemini model:").dim());
        println!();

        let current_config = self.load_config()?;
        let models = get_available_models();
        let model_names: Vec<&str> = models.iter().map(|m| m.name).collect();

        // Find current model index for default selection
        let current_index = models.iter()
            .position(|m| m.id == current_config.default_model)
            .unwrap_or(0);

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select your preferred Gemini model")
            .default(current_index)
            .items(&model_names)
            .interact()
            .map_err(|e| format!("Failed to get user selection: {}", e))?;

        let selected_model = models[selection].id.to_string();
        
        println!();
        if selected_model == current_config.default_model {
            println!("{} No change - keeping: {}", 
                style("ℹ").blue().bold(), 
                style(&selected_model).cyan().bold()
            );
        } else {
            println!("{} Changed from {} to {}", 
                style("✓").green().bold(),
                style(&current_config.default_model).dim(),
                style(&selected_model).cyan().bold()
            );
        }
        println!();

        let mut config = current_config;
        config.default_model = selected_model;

        self.save_config(&config)?;
        Ok(config)
    }

    pub fn change_response_length(&self) -> Result<Config, String> {
        println!("\\n{}", style("🔧 Change Response Length").bold().cyan());
        println!("{}", style("Select your preferred response length:").dim());
        println!();

        let current_config = self.load_config()?;
        let response_options = vec![
            ("brief", "Brief - Concise, essential information only"),
            ("balanced", "Balanced - Moderate detail with key information"),
            ("detailed", "Detailed - Comprehensive explanations with context"),
            ("verbose", "Verbose - Very thorough with examples and additional info")
        ];
        
        let option_names: Vec<&str> = response_options.iter().map(|(_, name)| *name).collect();
        
        // Find current response length index for default selection
        let current_index = response_options.iter()
            .position(|(id, _)| *id == current_config.response_length)
            .unwrap_or(1); // Default to "balanced"

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select your preferred response length")
            .default(current_index)
            .items(&option_names)
            .interact()
            .map_err(|e| format!("Failed to get user selection: {}", e))?;

        let selected_length = response_options[selection].0.to_string();
        
        println!();
        if selected_length == current_config.response_length {
            println!("{} No change - keeping: {}", 
                style("ℹ").blue().bold(), 
                style(&selected_length).cyan().bold()
            );
        } else {
            println!("{} Changed from {} to {}", 
                style("✓").green().bold(),
                style(&current_config.response_length).dim(),
                style(&selected_length).cyan().bold()
            );
        }
        println!();

        let mut config = current_config;
        config.response_length = selected_length;

        self.save_config(&config)?;
        Ok(config)
    }

    pub fn set_model(&self, model: &str) -> Result<Config, String> {
        // Validate that the model is in our list
        let models = get_available_models();
        let valid_model = models.iter().find(|m| m.id == model);
        
        if valid_model.is_none() {
            return Err(format!("Invalid model '{}'. Use --model without a value to see available models.", model));
        }

        let mut config = self.load_config()?;
        config.default_model = model.to_string();

        self.save_config(&config)?;
        
        println!("{} Default model changed to: {}", 
            style("✓").green().bold(),
            style(model).cyan().bold()
        );
        
        Ok(config)
    }

    pub fn show_current_model(&self) -> Result<(), String> {
        let config = self.load_config()?;
        println!("{} Current default model: {}", 
            style("ℹ").blue().bold(),
            style(&config.default_model).cyan().bold()
        );
        Ok(())
    }

    pub fn run_first_time_setup(&self) -> Result<Config, String> {
        println!("\\n{}", style("🚀 Welcome to huh!").bold().cyan());
        println!("{}", style("Let's set up your default Gemini model.").dim());
        println!();

        let models = get_available_models();
        let model_names: Vec<&str> = models.iter().map(|m| m.name).collect();

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select your preferred Gemini model")
            .default(0)
            .items(&model_names)
            .interact()
            .map_err(|e| format!("Failed to get user selection: {}", e))?;

        let selected_model = models[selection].id.to_string();
        
        println!();
        println!("{} Selected: {}", 
            style("✓").green().bold(), 
            style(&selected_model).cyan().bold()
        );
        println!("{}", style("You can change this anytime with the --model flag.").dim());
        println!();

        let mut config = Config::default();
        config.default_model = selected_model;

        self.save_config(&config)?;
        Ok(config)
    }

    pub fn change_temperature(&self) -> Result<Config, String> {
        println!("\\n{}", style("🔧 Change Temperature").bold().cyan());
        println!("{}", style("Temperature controls creativity (0.0 = focused, 1.0 = creative):").dim());
        println!();

        let current_config = self.load_config()?;
        let temp_options = vec![
            (0.0, "0.0 - Very focused and deterministic"),
            (0.3, "0.3 - Slightly focused"),
            (0.5, "0.5 - Balanced"),
            (0.7, "0.7 - Creative (recommended)"),
            (0.9, "0.9 - Very creative"),
            (1.0, "1.0 - Maximum creativity")
        ];
        
        let option_names: Vec<&str> = temp_options.iter().map(|(_, name)| *name).collect();
        let current_index = temp_options.iter()
            .position(|(temp, _)| (*temp - current_config.temperature).abs() < 0.01)
            .unwrap_or(3);

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select temperature setting")
            .default(current_index)
            .items(&option_names)
            .interact()
            .map_err(|e| format!("Failed to get user selection: {}", e))?;

        let selected_temp = temp_options[selection].0;
        
        println!();
        if (selected_temp - current_config.temperature).abs() < 0.01 {
            println!("{} No change - keeping: {}", 
                style("ℹ").blue().bold(), 
                style(&selected_temp.to_string()).cyan().bold()
            );
        } else {
            println!("{} Changed from {} to {}", 
                style("✓").green().bold(),
                style(&current_config.temperature.to_string()).dim(),
                style(&selected_temp.to_string()).cyan().bold()
            );
        }
        println!();

        let mut config = current_config;
        config.temperature = selected_temp;

        self.save_config(&config)?;
        Ok(config)
    }

    pub fn change_max_tokens(&self) -> Result<Config, String> {
        println!("\\n{}", style("🔧 Change Max Output Tokens").bold().cyan());
        println!("{}", style("Maximum number of tokens in AI responses:").dim());
        println!();

        let current_config = self.load_config()?;
        let token_options = vec![
            (1024, "1024 - Short responses"),
            (2048, "2048 - Medium responses"),
            (4096, "4096 - Long responses"),
            (8192, "8192 - Very long responses (recommended)"),
            (16384, "16384 - Maximum length responses")
        ];
        
        let option_names: Vec<&str> = token_options.iter().map(|(_, name)| *name).collect();
        let current_index = token_options.iter()
            .position(|(tokens, _)| *tokens == current_config.max_output_tokens)
            .unwrap_or(3);

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select max output tokens")
            .default(current_index)
            .items(&option_names)
            .interact()
            .map_err(|e| format!("Failed to get user selection: {}", e))?;

        let selected_tokens = token_options[selection].0;
        
        println!();
        if selected_tokens == current_config.max_output_tokens {
            println!("{} No change - keeping: {}", 
                style("ℹ").blue().bold(), 
                style(&selected_tokens.to_string()).cyan().bold()
            );
        } else {
            println!("{} Changed from {} to {}", 
                style("✓").green().bold(),
                style(&current_config.max_output_tokens.to_string()).dim(),
                style(&selected_tokens.to_string()).cyan().bold()
            );
        }
        println!();

        let mut config = current_config;
        config.max_output_tokens = selected_tokens;

        self.save_config(&config)?;
        Ok(config)
    }

    pub fn toggle_thinking(&self) -> Result<Config, String> {
        println!("\\n{}", style("🔧 Toggle Thinking Process Display").bold().cyan());
        println!("{}", style("Show AI reasoning and thought process:").dim());
        println!();

        let current_config = self.load_config()?;
        let new_value = !current_config.show_thinking;
        
        println!("{} Changed from {} to {}", 
            style("✓").green().bold(),
            style(&current_config.show_thinking.to_string()).dim(),
            style(&new_value.to_string()).cyan().bold()
        );
        println!();

        let mut config = current_config;
        config.show_thinking = new_value;

        self.save_config(&config)?;
        Ok(config)
    }

    pub fn toggle_auto_save(&self) -> Result<Config, String> {
        println!("\\n{}", style("🔧 Toggle Auto-Save History").bold().cyan());
        println!("{}", style("Automatically save conversation history:").dim());
        println!();

        let current_config = self.load_config()?;
        let new_value = !current_config.auto_save_history;
        
        println!("{} Changed from {} to {}", 
            style("✓").green().bold(),
            style(&current_config.auto_save_history.to_string()).dim(),
            style(&new_value.to_string()).cyan().bold()
        );
        println!();

        let mut config = current_config;
        config.auto_save_history = new_value;

        self.save_config(&config)?;
        Ok(config)
    }

    pub fn change_shell(&self) -> Result<Config, String> {
        println!("\\n{}", style("🔧 Change Default Shell").bold().cyan());
        println!("{}", style("Default shell for command execution:").dim());
        println!();

        let current_config = self.load_config()?;
        let shell_options = vec![
            ("bash", "Bash - Most common Unix shell"),
            ("zsh", "Zsh - Feature-rich shell with plugins"),
            ("fish", "Fish - User-friendly shell with syntax highlighting"),
            ("sh", "Sh - POSIX-compliant shell"),
            ("powershell", "PowerShell - Windows/cross-platform shell"),
            ("cmd", "CMD - Windows command prompt")
        ];
        
        let option_names: Vec<&str> = shell_options.iter().map(|(_, name)| *name).collect();
        let current_index = shell_options.iter()
            .position(|(shell, _)| *shell == current_config.default_shell)
            .unwrap_or(0);

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select default shell")
            .default(current_index)
            .items(&option_names)
            .interact()
            .map_err(|e| format!("Failed to get user selection: {}", e))?;

        let selected_shell = shell_options[selection].0.to_string();
        
        println!();
        if selected_shell == current_config.default_shell {
            println!("{} No change - keeping: {}", 
                style("ℹ").blue().bold(), 
                style(&selected_shell).cyan().bold()
            );
        } else {
            println!("{} Changed from {} to {}", 
                style("✓").green().bold(),
                style(&current_config.default_shell).dim(),
                style(&selected_shell).cyan().bold()
            );
        }
        println!();

        let mut config = current_config;
        config.default_shell = selected_shell;

        self.save_config(&config)?;
        Ok(config)
    }

    pub fn change_timeout(&self) -> Result<Config, String> {
        println!("\\n{}", style("🔧 Change API Timeout").bold().cyan());
        println!("{}", style("Timeout for API requests in seconds:").dim());
        println!();

        let current_config = self.load_config()?;
        let timeout_options = vec![
            (10, "10 seconds - Quick timeout"),
            (20, "20 seconds - Short timeout"),
            (30, "30 seconds - Standard timeout (recommended)"),
            (60, "60 seconds - Long timeout"),
            (120, "120 seconds - Extended timeout"),
            (300, "300 seconds - Maximum timeout")
        ];
        
        let option_names: Vec<&str> = timeout_options.iter().map(|(_, name)| *name).collect();
        let current_index = timeout_options.iter()
            .position(|(timeout, _)| *timeout == current_config.api_timeout)
            .unwrap_or(2);

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select API timeout")
            .default(current_index)
            .items(&option_names)
            .interact()
            .map_err(|e| format!("Failed to get user selection: {}", e))?;

        let selected_timeout = timeout_options[selection].0;
        
        println!();
        if selected_timeout == current_config.api_timeout {
            println!("{} No change - keeping: {} seconds", 
                style("ℹ").blue().bold(), 
                style(&selected_timeout.to_string()).cyan().bold()
            );
        } else {
            println!("{} Changed from {} to {} seconds", 
                style("✓").green().bold(),
                style(&current_config.api_timeout.to_string()).dim(),
                style(&selected_timeout.to_string()).cyan().bold()
            );
        }
        println!();

        let mut config = current_config;
        config.api_timeout = selected_timeout;

        self.save_config(&config)?;
        Ok(config)
    }
}

pub struct GeminiModel {
    pub id: &'static str,
    pub name: &'static str,
}

pub fn get_available_models() -> Vec<GeminiModel> {
    vec![
        GeminiModel {
            id: "gemini-2.0-flash",
            name: "Gemini 2.0 Flash (Recommended) - Fast, balanced performance",
        },
        GeminiModel {
            id: "gemini-1.5-pro",
            name: "Gemini 1.5 Pro - Advanced reasoning and complex tasks",
        },
        GeminiModel {
            id: "gemini-1.5-flash",
            name: "Gemini 1.5 Flash - Fast responses, good for quick queries",
        },
        GeminiModel {
            id: "gemini-1.5-flash-8b",
            name: "Gemini 1.5 Flash 8B - Lightweight, very fast responses",
        },
        GeminiModel {
            id: "gemini-2.0-flash-exp",
            name: "Gemini 2.0 Flash Experimental - Latest features (may be unstable)",
        },
        GeminiModel {
            id: "gemini-exp-1121",
            name: "Gemini Experimental 1121 - Cutting-edge experimental model",
        },
        GeminiModel {
            id: "gemini-exp-1206",
            name: "Gemini Experimental 1206 - Latest experimental features",
        },
        GeminiModel {
            id: "gemini-2.5-flash-lite-preview-06-17", 
            name: "Gemini 2.5 Flash Lite Preview - Optimized for efficiency",
        },
    ]
}