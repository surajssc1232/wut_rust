use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use dialoguer::{Select, theme::ColorfulTheme};
use console::style;

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub default_model: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            default_model: "gemini-2.0-flash".to_string(),
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

        serde_json::from_str(&config_content)
            .map_err(|e| format!("Failed to parse config file: {}", e))
    }

    pub fn save_config(&self, config: &Config) -> Result<(), String> {
        let config_json = serde_json::to_string_pretty(config)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;

        fs::write(&self.config_path, config_json)
            .map_err(|e| format!("Failed to write config file: {}", e))
    }

    pub fn change_model(&self) -> Result<Config, String> {
        println!("\n{}", style("🔧 Change Default Model").bold().cyan());
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

        let config = Config {
            default_model: selected_model,
        };

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

        let config = Config {
            default_model: model.to_string(),
        };

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
        println!("\n{}", style("🚀 Welcome to huh!").bold().cyan());
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

        let config = Config {
            default_model: selected_model,
        };

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