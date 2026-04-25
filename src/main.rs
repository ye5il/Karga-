mod config;
mod rss;
mod ui;
mod wiki;
mod notify;

use std::io::Write;
use std::sync::Arc;
use tokio::sync::Mutex;

use config::Config;
use rss::RssFetcher;
use wiki::WikiFetcher;
use notify::NotificationManager;

fn print_ascii() {
    println!(r#"
 ▄█   ▄█▄    ▄████████    ▄████████    ▄██████▄     ▄████████
  ███ ▄███▀   ███    ███   ███    ███   ███    ███   ███    ███
  ███▐██▀     ███    ███   ███    ███   ███    █▀    ███    ███
 ▄█████▀      ███    ███  ▄███▄▄▄▄██▀  ▄███          ███    ███
▀▀█████▄    ▀███████████ ▀▀███▀▀▀▀▀   ▀▀███ ████▄  ▀███████████
  ███▐██▄     ███    ███ ▀███████████   ███    ███   ███    ███
  ███ ▀███▄   ███    ███   ███    ███   ███    ███   ███    ███
  ███   ▀█▀   ███    █▀    ███    ███   ████████▀    ███    █▀ 
  ▀                        ███    ███
"#);
    std::io::stdout().flush().ok();
    std::thread::sleep(std::time::Duration::from_millis(1500));
    print!("\x1b[2J\x1b[H");
    std::io::stdout().flush().ok();
}

#[tokio::main]
async fn main() {
    print_ascii();
    
    let config = match Config::load_or_initialize().await {
        Ok(c) => c,
        Err(e) => {
            eprintln!("[Error] Config: {}", e);
            return;
        }
    };
    
    let rss_fetcher = Arc::new(Mutex::new(RssFetcher::new()));
    let wiki_fetcher = Arc::new(Mutex::new(WikiFetcher::new()));
    let notifier = NotificationManager::new();
    
    if let Err(e) = ui::run_app(config, rss_fetcher, wiki_fetcher, notifier).await {
        eprintln!("[Error] UI: {}", e);
    }
}