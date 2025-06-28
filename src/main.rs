mod prompt;
mod shell;
mod history;
mod gemini;

use history::HistoryManager;
use gemini::GeminiClient;
use std::env;
use std::io::{self, Write};
use std::time::Duration;
use tokio::sync::oneshot;
use tokio::select;

async fn loading_animation(mut rx: oneshot::Receiver<()>) {
    let frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    let mut i = 0;
    print!("\x1b[?25l"); // Hide cursor
    loop {
        select! {
            _ = tokio::time::sleep(Duration::from_millis(80)) => {
                print!("\r{} Analyzing...\x1b[K", frames[i % frames.len()]);
                io::stdout().flush().unwrap();
                i += 1;
            }
            _ = &mut rx => {
                break;
            }
        }
    }
    print!("\r\x1b[K"); // Clear line
    print!("\x1b[?25h"); // Show cursor
    io::stdout().flush().unwrap();
}

async fn handle_wut_command() {
    let history_manager = HistoryManager::new().unwrap();
    let commands = history_manager.get_last_commands(2).unwrap();

    if commands.is_empty() {
        println!("No commands found in history.");
        return;
    }

    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY not set");
    let client = GeminiClient::new(api_key);

    let (tx, rx) = oneshot::channel();
    let animation_handle = tokio::spawn(loading_animation(rx));

    match client.analyze_commands(&commands).await {
        Ok((analysis_text, suggestion)) => {
            let _ = tx.send(()); // Stop animation
            animation_handle.await.unwrap(); // Wait for animation to finish clearing

            for char_code in analysis_text.chars() {
                print!("{}", char_code);
                io::stdout().flush().unwrap();
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
            println!(); // Newline after animation

            if let Some(sugg) = suggestion {
                println!("Do you want to run this command? (Y/n)");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");
                let input = input.trim().to_lowercase();
                if input == "y" || input == "" {
                    shell::execute_command(&sugg);
                }
            }
        },
        Err(e) => {
            let _ = tx.send(()); // Stop animation on error as well
            animation_handle.await.unwrap();
            eprintln!("Error analyzing commands: {}", e);
        }
    }
}

#[tokio::main]
async fn main() {
    handle_wut_command().await;
}
