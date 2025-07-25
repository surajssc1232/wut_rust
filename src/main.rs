mod config;
mod gemini;
mod history;
mod prompt;
mod shell;

use clap::{Arg, Command};
use config::ConfigManager;
use gemini::GeminiClient;
use history::HistoryManager;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::time::Duration;
use tokio::select;
use tokio::sync::oneshot;

async fn loading_animation(mut rx: oneshot::Receiver<()>) {
    let frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    let mut i = 0;
    print!("\x1b[?25l");
    loop {
        select! {
            _ = tokio::time::sleep(Duration::from_millis(20)) => {
                print!("\r{} Analyzing...\x1b[K", frames[i % frames.len()]);
                io::stdout().flush().unwrap();
                i += 1;
            }
            _ = &mut rx => {
                break;
            }
        }
    }
    print!("\r\x1b[K");
    print!("\x1b[?25h");
    io::stdout().flush().unwrap();
}

async fn handle_wut_command(api_key: String, model: String, config: &config::Config) {
    let history_manager = HistoryManager::new().unwrap();
    let commands = history_manager.get_last_commands(2).unwrap();

    if commands.is_empty() {
        println!("No commands found in history.");
        return;
    }

    let client = GeminiClient::new(api_key, model, config);

    let (tx, rx) = oneshot::channel();
    let animation_handle = tokio::spawn(loading_animation(rx));

    match client.analyze_commands(&commands).await {
        Ok((analysis_text, _suggestion)) => {
            let _ = tx.send(());
            animation_handle.await.unwrap();

            print!("{}", analysis_text);
            println!();
            io::stdout().flush().unwrap();
        }
        Err(e) => {
            let _ = tx.send(());
            animation_handle.await.unwrap();
            eprintln!("Error analyzing commands: {}", e);
        }
    }
}

async fn handle_write_command(file_path: String, context: String, api_key: String, model: String, config: &config::Config) {
    let client = GeminiClient::new(api_key, model, config);

    let (tx, rx) = oneshot::channel();
    let animation_handle = tokio::spawn(loading_animation(rx));

    match client.write_or_edit_file(&file_path, &context).await {
        Ok(()) => {
            let _ = tx.send(());
            animation_handle.await.unwrap();
            println!("✓ File {} has been successfully written/edited!", file_path);
        }
        Err(e) => {
            let _ = tx.send(());
            animation_handle.await.unwrap();
            eprintln!("Error writing/editing file: {}", e);
        }
    }
}

async fn handle_query_command(query: String, api_key: String, model: String, config: &config::Config) {
    let client = GeminiClient::new(api_key, model, config);

    let (tx, rx) = oneshot::channel();
    let animation_handle = tokio::spawn(loading_animation(rx));

    match client.query_gemini(&query).await {
        Ok(response_text) => {
            let _ = tx.send(());
            animation_handle.await.unwrap();

            for char_code in response_text.chars() {
                print!("{}", char_code);
                io::stdout().flush().unwrap();
                tokio::time::sleep(Duration::from_millis(3)).await;
            }
            println!();
        }
        Err(e) => {
            let _ = tx.send(());
            animation_handle.await.unwrap();
            eprintln!("Error querying Gemini: {}", e);
        }
    }
}

#[tokio::main]
async fn main() {
    let matches = Command::new("huh")
        .version("0.2.0")
        .about("AI-powered shell command analysis tool")
        .arg(
            Arg::new("api-key")
                .long("api-key")
                .short('k')
                .value_name("KEY")
                .help("Google Gemini API key (overrides GEMINI_API_KEY env var)"),
        )
        .arg(
            Arg::new("model")
                .long("model")
                .short('m')
                .value_name("MODEL")
                .num_args(0..=1)
                .default_missing_value("")
                .help("Set default model (persistent) or show model selection menu if no value given"),
        )
        .arg(
            Arg::new("model-now")
                .long("model-now")
                .short('n')
                .action(clap::ArgAction::SetTrue)
                .help("Show currently configured default model"),
        )
        .arg(
            Arg::new("config")
                .long("config")
                .short('c')
                .action(clap::ArgAction::SetTrue)
                .help("Open interactive configuration menu"),
        )
        .arg(
            Arg::new("show-config")
                .long("show-config")
                .action(clap::ArgAction::SetTrue)
                .help("Show current configuration"),
        )
        .arg(
            Arg::new("write")
                .long("write")
                .short('w')
                .action(clap::ArgAction::SetTrue)
                .help("Write/edit mode - use with @<file> <context>"),
        )
        .arg(
            Arg::new("query")
                .help("Query to send to Gemini")
                .num_args(0..)
                .trailing_var_arg(true),
        )
        .get_matches();

    // Initialize config manager
    let config_manager = ConfigManager::new().expect("Failed to initialize config manager");
    
    // Handle --model-now flag
    if matches.get_flag("model-now") {
        config_manager.show_current_model().expect("Failed to show current model");
        return;
    }

    // Handle --show-config flag
    if matches.get_flag("show-config") {
        let config = config_manager.load_config().expect("Failed to load configuration");
        println!("Current configuration:");
        println!("  Default model: {}", config.default_model);
        println!("  Response length: {}", config.response_length);
        println!("  Temperature: {}", config.temperature);
        println!("  Max output tokens: {}", config.max_output_tokens);
        println!("  Show thinking: {}", config.show_thinking);
        println!("  Auto-save history: {}", config.auto_save_history);
        println!("  Default shell: {}", config.default_shell);
        println!("  API timeout: {} seconds", config.api_timeout);
        return;
    }

    // Handle -c/--config flag
    if matches.get_flag("config") {
        config_manager.interactive_config_menu().expect("Failed to open configuration menu");
        return;
    }
    
    // Handle --model flag behavior
    if let Some(model_value) = matches.get_one::<String>("model") {
        if model_value.is_empty() {
            // --model without value: show interactive menu
            config_manager.change_model().expect("Failed to change model");
            return;
        } else {
            // --model with value: set as new default
            match config_manager.set_model(model_value) {
                Ok(_) => return,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }
    
    // For regular operation, we need an API key
    let api_key = matches
        .get_one::<String>("api-key")
        .cloned()
        .or_else(|| env::var("GEMINI_API_KEY").ok())
        .expect(
            "API key must be provided via --api-key flag or GEMINI_API_KEY environment variable",
        );
    
    // Check if this is the first run and run setup if needed
    let config = if !config_manager.config_exists() {
        config_manager.run_first_time_setup()
            .expect("Failed to run first-time setup")
    } else {
        config_manager.load_config()
            .expect("Failed to load configuration")
    };

    // Use the configured default model
    let model = config.default_model.clone();

    let write_mode = matches.get_flag("write");

    if let Some(query_args) = matches.get_many::<String>("query") {
        let query_vec: Vec<&str> = query_args.map(|s| s.as_str()).collect();

        if !query_vec.is_empty() {
            let first_arg = query_vec[0];
            if first_arg.starts_with('@') {
                let file_path = &first_arg[1..];

                if write_mode {
                    // Write/edit mode: huh -w @file context
                    if query_vec.len() > 1 {
                        let context = query_vec[1..].join(" ");
                        handle_write_command(file_path.to_string(), context, api_key, model, &config).await;
                    } else {
                        eprintln!(
                            "Error: Write mode requires context. Usage: huh -w @<file> <context>"
                        );
                    }
                } else {
                    // Query mode: huh @file context (existing behavior)
                    match fs::read_to_string(file_path) {
                        Ok(file_content) => {
                            let mut query = format!(
                                "Content from {}:\n---\n{}\n---\n",
                                file_path, file_content
                            );
                            if query_vec.len() > 1 {
                                query.push_str(&query_vec[1..].join(" "));
                            }
                            handle_query_command(query, api_key, model, &config).await;
                        }
                        Err(e) => {
                            eprintln!("Error reading file {}: {}", file_path, e);
                        }
                    }
                }
            } else {
                if write_mode {
                    eprintln!("Error: Write mode requires a file path starting with @. Usage: huh -w @<file> <context>");
                } else {
                    let query = query_vec.join(" ");
                    handle_query_command(query, api_key, model, &config).await;
                }
            }
        } else {
            if write_mode {
                eprintln!("Error: Write mode requires arguments. Usage: huh -w @<file> <context>");
            } else {
                handle_wut_command(api_key, model, &config).await;
            }
        }
    } else {
        if write_mode {
            eprintln!("Error: Write mode requires arguments. Usage: huh -w @<file> <context>");
        } else {
            handle_wut_command(api_key, model, &config).await;
        }
    }
}
