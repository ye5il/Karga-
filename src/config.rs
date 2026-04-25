use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::PathBuf;

pub const CATEGORIES: &[&str] = &[
    "Haber",
    "Dünya",
    "Spor",
    "Politika",
    "Ekonomi",
    "Teknoloji",
    "Kültür",
    "Sağlık",
    "Bilim",
    "Güvenlik",
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub keywords: Vec<String>,
    pub rss_feeds: Vec<RssFeed>,
    pub scan_interval_secs: u64,
    pub theme: Theme,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RssFeed {
    pub name: String,
    pub url: String,
    pub region: String,
    pub category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub bg_dark_blue: String,
    pub bg_dark_gray: String,
    pub black: String,
    pub accent: String,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            bg_dark_blue: "#0a1628".to_string(),
            bg_dark_gray: "#1c2128".to_string(),
            black: "#000000".to_string(),
            accent: "#4a90d9".to_string(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            keywords: Vec::new(),
            rss_feeds: vec![
                RssFeed { name: "NTV".to_string(), url: "https://www.ntv.com.tr/gundem.rss".to_string(), region: "tr".to_string(), category: "Haber".to_string() },
                RssFeed { name: "Hürriyet".to_string(), url: "http://www.hurriyet.com.tr/rss/gundem".to_string(), region: "tr".to_string(), category: "Haber".to_string() },
                RssFeed { name: "Milliyet".to_string(), url: "http://www.milliyet.com.tr/rss/rssNew/gundemRss.xml".to_string(), region: "tr".to_string(), category: "Haber".to_string() },
                RssFeed { name: "Sabah".to_string(), url: "https://www.sabah.com.tr/rss/gundem.xml".to_string(), region: "tr".to_string(), category: "Haber".to_string() },
                RssFeed { name: "Habertürk".to_string(), url: "http://www.haberturk.com/rss".to_string(), region: "tr".to_string(), category: "Haber".to_string() },
                RssFeed { name: "BBC Türkçe".to_string(), url: "http://feeds.bbci.co.uk/turkce/rss.xml".to_string(), region: "tr".to_string(), category: "Dünya".to_string() },
                RssFeed { name: "Sputnik".to_string(), url: "https://tr.sputniknews.com/export/rss2/archive/index.xml".to_string(), region: "tr".to_string(), category: "Politika".to_string() },
                RssFeed { name: "BBC".to_string(), url: "https://feeds.bbci.co.uk/news/world/rss.xml".to_string(), region: "global".to_string(), category: "Dünya".to_string() },
                RssFeed { name: "Al Jazeera".to_string(), url: "https://www.aljazeera.com/xml/rss/all.xml".to_string(), region: "global".to_string(), category: "Dünya".to_string() },
                RssFeed { name: "CNN".to_string(), url: "http://rss.cnn.com/rss/edition.rss".to_string(), region: "global".to_string(), category: "Dünya".to_string() },
                RssFeed { name: "Wired".to_string(), url: "https://www.wired.com/feed/rss".to_string(), region: "global".to_string(), category: "Teknoloji".to_string() },
                RssFeed { name: "CNBC Economy".to_string(), url: "https://www.cnbc.com/id/20910258/device/rss/rss.html".to_string(), region: "global".to_string(), category: "Ekonomi".to_string() },
            ],
            scan_interval_secs: 300,
            theme: Theme::default(),
        }
    }
}

impl Config {
    fn config_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("karga")
            .join("config.json")
    }

    pub async fn load_or_initialize() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let path = Self::config_path();
        
        if path.exists() {
            let content = std::fs::read_to_string(&path)?;
            let mut config: Config = serde_json::from_str(&content)?;
            
            let mut default_urls: HashSet<String> = Config::default()
                .rss_feeds
                .iter()
                .map(|f| f.url.clone())
                .collect();
            
            for feed in &config.rss_feeds {
                default_urls.remove(&feed.url);
            }
            
            let default_config = Config::default();
            for feed in default_config.rss_feeds.iter() {
                if default_urls.contains(&feed.url) {
                    config.rss_feeds.push(feed.clone());
                }
            }
            
            Ok(config)
        } else {
            let keywords = crate::wiki::fetch_wikipedia_keywords().await?;
            let mut config = Config::default();
            config.keywords = keywords;
            
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            
            let content = serde_json::to_string_pretty(&config)?;
            std::fs::write(&path, content)?;
            
            println!("[Karga] Initialized with {} keywords from Wikipedia", config.keywords.len());
            
            Ok(config)
        }
    }
}