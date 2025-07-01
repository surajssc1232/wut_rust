mod gemini;
mod history;
mod prompt;
mod shell;

use clap::{Arg, Command};
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
            _ = tokio::time::sleep(Duration::from_millis(40)) => {
                print!("\r{} Thinking...\x1b[K", frames[i % frames.len()]);
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

async fn handle_wut_command(api_key: String, model: String) {
    let history_manager = HistoryManager::new().unwrap();
    let commands = history_manager.get_last_commands(2).unwrap();

    if commands.is_empty() {
        println!("No commands found in history.");
        return;
    }

    let client = GeminiClient::new(api_key, model);

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

async fn handle_write_command(file_path: String, context: String, api_key: String, model: String) {
    let client = GeminiClient::new(api_key, model);

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

async fn handle_query_command(query: String, api_key: String, model: String) {
    let client = GeminiClient::new(api_key, model);

    let (tx, rx) = oneshot::channel();
    let animation_handle = tokio::spawn(loading_animation(rx));

    match client.query_gemini(&query).await {
        Ok(response_text) => {
            let _ = tx.send(());
            animation_handle.await.unwrap();

            for char_code in response_text.chars() {
                print!("{}", char_code);
                io::stdout().flush().unwrap();
                tokio::time::sleep(Duration::from_millis(1)).await;
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
                .default_value("gemini-2.5-flash-lite-preview-06-17")
                .help("Gemini model to use"),
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

    let api_key = matches
        .get_one::<String>("api-key")
        .cloned()
        .or_else(|| env::var("GEMINI_API_KEY").ok())
        .expect(
            "API key must be provided via --api-key flag or GEMINI_API_KEY environment variable",
        );

    let model = matches
        .get_one::<String>("model")
        .cloned()
        .unwrap_or_else(|| "gemini-2.5-flash-lite-preview-06-17".to_string());

    let write_mode = matches.get_flag("write");

    if let Some(query_args) = matches.get_many::<String>("query") {
        let query_vec: Vec<&str> = query_args.map(|s| s.as_str()).collect();

        if !query_vec.is_empty() {
            let first_arg = query_vec[0];
            if first_arg.starts_with('@') {
                let file_path = &first_arg[1..];
                
                if write_mode {
                    if query_vec.len() > 1 {
                        let context = query_vec[1..].join(" ");
                        handle_write_command(file_path.to_string(), context, api_key, model).await;
                    } else {
                        eprintln!("Error: Write mode requires context. Usage: huh -w @<file> <context>");
                    }
                } else {
                    match fs::read_to_string(file_path) {
                        Ok(file_content) => {
                            let mut query =
                                format!("Content from {}:\n---\n{}\n---\n", file_path, file_content);
                            if query_vec.len() > 1 {
                                query.push_str(&query_vec[1..].join(" "));
                            }
                            handle_query_command(query, api_key, model).await;
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
                    handle_query_command(query, api_key, model).await;
                }
            }
        } else {
            if write_mode {
                eprintln!("Error: Write mode requires arguments. Usage: huh -w @<file> <context>");
            } else {
                handle_wut_command(api_key, model).await;
            }
        }
    } else {
        if write_mode {
            eprintln!("Error: Write mode requires arguments. Usage: huh -w @<file> <context>");
        } else {
            handle_wut_command(api_key, model).await;
        }
    }
}