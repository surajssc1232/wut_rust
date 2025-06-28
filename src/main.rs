mod prompt;
mod shell;
mod history;
mod gemini;

use history::HistoryManager;
use gemini::GeminiClient;
use std::env;


async fn handle_wut_command() {
    let history_manager = HistoryManager::new().unwrap();
    let commands = history_manager.get_last_commands(2).unwrap();

    if commands.is_empty() {
        println!("No commands found in history.");
        return;
    }

    
    

    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY not set");
    let client = GeminiClient::new(api_key);

    match client.analyze_commands(&commands).await {
        Ok(analysis) => println!("{}", analysis),
        Err(e) => eprintln!("Error analyzing commands: {}", e),
    }
}

#[tokio::main]
async fn main() {
    handle_wut_command().await;
}
