mod constants;
mod gemini;
mod history;
mod prompt;
mod shell;
mod shell_detection;

use gemini::GeminiClient;
use history::HistoryManager;
use std::env;
use std::io::{self, Write};
use std::fs;
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

async fn handle_wut_command() {
    let history_manager = match HistoryManager::new() {
        Ok(manager) => manager,
        Err(e) => {
            eprintln!("Error initializing history manager: {}", e);
            return;
        }
    };
    
    let commands = match history_manager.get_last_commands(2) {
        Ok(commands) => commands,
        Err(e) => {
            eprintln!("Error getting command history: {}", e);
            return;
        }
    };

    if commands.is_empty() {
        println!("No commands found in history.");
        return;
    }

    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY not set");
    let client = GeminiClient::new(api_key);

    let (tx, rx) = oneshot::channel();
    let animation_handle = tokio::spawn(loading_animation(rx));

    match client.analyze_commands(&commands).await {
        Ok((analysis_text, _suggestion)) => {
            let _ = tx.send(());
            animation_handle.await.unwrap();

            for char_code in analysis_text.chars() {
                print!("{}", char_code);
                io::stdout().flush().unwrap();
                tokio::time::sleep(Duration::from_millis(3)).await;
            }
            println!();
        }
        Err(e) => {
            let _ = tx.send(());
            animation_handle.await.unwrap();
            eprintln!("Error analyzing commands: {}", e);
        }
    }
}

async fn handle_query_command(query: String) {
    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY not set");
    let client = GeminiClient::new(api_key);

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
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let first_arg = &args[1];
        if first_arg.starts_with('@') {
            let file_path = &first_arg[1..];
            match fs::read_to_string(file_path) {
                Ok(file_content) => {
                    let mut query = format!("Content from {}:
---
{}
---
", file_path, file_content);
                    if args.len() > 2 {
                        query.push_str(&args[2..].join(" "));
                    }
                    handle_query_command(query).await;
                }
                Err(e) => {
                    eprintln!("Error reading file {}: {}", file_path, e);
                }
            }
        } else {
            let query = args[1..].join(" ");
            handle_query_command(query).await;
        }
    } else {
        handle_wut_command().await;
    }
}
