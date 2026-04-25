use std::fs::OpenOptions;
use std::io::Write;
use reqwest::Client;
use atom_syndication::Feed;

fn log_to_file(msg: &str) {
    if let Ok(mut f) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("/home/cinar/Desktop/karga/debug.log")
    {
        let _ = writeln!(f, "[{}] {}", chrono::Local::now().format("%H:%M:%S"), msg);
    }
}

pub struct RssFetcher {
    client: Client,
}

#[derive(Debug, Clone)]
pub struct NewsItem {
    pub title: String,
    pub link: String,
    pub description: String,
    pub pub_date: String,
    pub source: String,
    pub category: String,
}

impl RssFetcher {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .no_gzip()
                .redirect(reqwest::redirect::Policy::limited(10))
                .user_agent("KargaNews/1.0")
                .build()
                .unwrap_or_else(|_| Client::new()),
        }
    }

    pub async fn fetch_feed(&self, url: &str, source_name: &str, category: &str) -> Result<Vec<NewsItem>, Box<dyn std::error::Error + Send + Sync>> {
        log_to_file(&format!("[RSS] fetch_feed called for {} = {}", source_name, url));
        
        let response = self.client.get(url).send().await?;
        let status = response.status();
        log_to_file(&format!("[RSS] {} status: {}", source_name, status));
        
        let bytes = response.bytes().await?;
        log_to_file(&format!("[RSS] {} got {} bytes", source_name, bytes.len()));
        
        let content = String::from_utf8_lossy(&bytes);
        
        let format_type = if content.contains("<rss") || content.contains("<channel>") {
            "RSS"
        } else if content.contains("<feed") || content.contains("xmlns=\"http://www.w3.org/2005/Atom\"") {
            "Atom"
        } else {
            log_to_file(&format!("[RSS] Unknown format for {}: {}", source_name, &content[..100.min(content.len())]));
            "UNKNOWN"
        };
        
        log_to_file(&format!("[RSS] {} format: {}", source_name, format_type));
        
        let items = if format_type == "RSS" {
            self.parse_rss(&content, source_name, category)
        } else if format_type == "Atom" {
            self.parse_atom(&content, source_name, category)
        } else {
            Ok(Vec::new())
        };
        
        items
    }
    
    fn parse_rss(&self, content: &str, source_name: &str, category: &str) -> Result<Vec<NewsItem>, Box<dyn std::error::Error + Send + Sync>> {
        let channel = rss::Channel::read_from(content.as_bytes())?;
        
        let items: Vec<NewsItem> = channel
            .items()
            .iter()
            .take(20)
            .map(|item| NewsItem {
                title: item.title().unwrap_or("").to_string(),
                link: item.link().unwrap_or("").to_string(),
                description: item.description().unwrap_or("").to_string(),
                pub_date: item.pub_date().unwrap_or("").to_string(),
                source: source_name.to_string(),
                category: category.to_string(),
            })
            .collect();
        
        Ok(items)
    }
    
    fn parse_atom(&self, content: &str, source_name: &str, category: &str) -> Result<Vec<NewsItem>, Box<dyn std::error::Error + Send + Sync>> {
        let feed = Feed::read_from(content.as_bytes())?;
        
        let items: Vec<NewsItem> = feed
            .entries()
            .iter()
            .take(20)
            .map(|entry| {
                let link = entry.links()
                    .first()
                    .map(|l| l.href().to_string())
                    .unwrap_or_default();
                
                let published = entry.published()
                    .or_else(|| Some(entry.updated()))
                    .map(|d| d.to_string())
                    .unwrap_or_default();
                
                NewsItem {
                    title: entry.title().to_string(),
                    link,
                    description: entry.summary().map(|s| s.to_string()).unwrap_or_default(),
                    pub_date: published,
                    source: source_name.to_string(),
                    category: category.to_string(),
                }
            })
            .collect();
        
        Ok(items)
    }

    pub async fn fetch_multiple(&self, feeds: &[(&str, &str, &str)]) -> Result<Vec<NewsItem>, Box<dyn std::error::Error + Send + Sync>> {
        let mut all_items = Vec::new();
        
        for (url, name, category) in feeds {
            log_to_file(&format!("[RSS] Fetching {} from {}", name, url));
            
            match self.fetch_feed(url, name, category).await {
                Ok(items) => {
                    log_to_file(&format!("[RSS] Got {} items from {}", items.len(), name));
                    all_items.extend(items);
                },
                Err(e) => {
                    log_to_file(&format!("[RSS] Failed to fetch from {}: {}", name, e));
                },
            }
        }
        
        all_items.sort_by(|a, b| b.pub_date.cmp(&a.pub_date));
        
        Ok(all_items)
    }
}