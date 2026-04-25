use reqwest::Client;
use scraper::{Html, Selector};
use regex::Regex;
use chrono::Datelike;

pub struct WikiFetcher {
    client: Client,
}

#[derive(Debug, Clone)]
pub struct WikiAnniversary {
    pub year: String,
    pub description: String,
}

impl WikiFetcher {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .user_agent("Karga News/0.1")
                .build()
                .unwrap_or_else(|_| Client::new()),
        }
    }

    pub async fn fetch_today_anniversaries(&self) -> Result<Vec<WikiAnniversary>, Box<dyn std::error::Error + Send + Sync>> {
        let month_names = ["January", "February", "March", "April", "May", "June",
                          "July", "August", "September", "October", "November", "December"];
        
        let now = chrono::Local::now();
        let month = month_names[now.month0() as usize];
        let day = now.day();
        
        let url = format!("https://en.wikipedia.org/wiki/Wikipedia:Selected_anniversaries/{}_{}", month, day);
        
        let response = self.client.get(&url).send().await?;
        let html = response.text().await?;
        
        let document = Html::parse_document(&html);
        let mut anniversaries = Vec::new();
        
        let list_selector = Selector::parse("ul, ol").unwrap();
        let item_selector = Selector::parse("li").unwrap();
        
        for list in document.select(&list_selector) {
            for item in list.select(&item_selector) {
                let text = item.text().collect::<String>();
                if let Some(year_match) = extract_year(&text) {
                    anniversaries.push(WikiAnniversary {
                        year: year_match,
                        description: clean_text(&text),
                    });
                }
            }
        }
        
        Ok(anniversaries)
    }

    pub async fn fetch_all_keywords(&self) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
        let mut all_keywords = Vec::new();
        
        let base_keywords = get_base_keywords();
        all_keywords.extend(base_keywords);
        
        let month_names = ["January", "February", "March", "April", "May", "June",
                      "July", "August", "September", "October", "November", "December"];
        
        for month in month_names.iter().take(3) {
            for day in 1..=28 {
                let url = format!("https://en.wikipedia.org/wiki/Wikipedia:Selected_anniversaries/{}_{}", month, day);
                
                if let Ok(response) = self.client.get(&url).send().await {
                    if let Ok(html) = response.text().await {
                        let keywords = extract_keywords_from_html(&html);
                        all_keywords.extend(keywords);
                    }
                }
            }
        }
        
        all_keywords.sort();
        all_keywords.dedup();
        
        Ok(all_keywords)
    }
}

fn extract_year(text: &str) -> Option<String> {
    let re = Regex::new(r"(\d{4})").unwrap();
    re.captures(text)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().to_string())
}

fn extract_keywords_from_html(html: &str) -> Vec<String> {
    let base_keywords = get_base_keywords();
    let html_lower = html.to_lowercase();
    
    base_keywords
        .into_iter()
        .filter(|kw| html_lower.contains(&kw.to_lowercase()))
        .collect()
}

fn clean_text(text: &str) -> String {
    let re = Regex::new(r"^\d{4}\s*[-–]\s*").unwrap();
    re.replace(text, "").to_string()
}

fn get_base_keywords() -> Vec<String> {
    vec![
        "invasion".to_string(),
        "military intervention".to_string(),
        "revolution".to_string(),
        "war".to_string(),
        "peace".to_string(),
        "crisis".to_string(),
        "famine".to_string(),
        "disaster".to_string(),
        "state".to_string(),
        "nation".to_string(),
        "empire".to_string(),
        "kingdom".to_string(),
        "republic".to_string(),
        "independence".to_string(),
        "treaty".to_string(),
        "agreement".to_string(),
        "battle".to_string(),
        "siege".to_string(),
        "attack".to_string(),
        "defeat".to_string(),
        "victory".to_string(),
        "occupation".to_string(),
        "liberation".to_string(),
        "genocide".to_string(),
        "holocaust".to_string(),
        "slavery".to_string(),
        "emancipation".to_string(),
        "protest".to_string(),
        "demonstration".to_string(),
        "riot".to_string(),
        "rebellion".to_string(),
        "coup".to_string(),
        "assassination".to_string(),
        "murder".to_string(),
        "execution".to_string(),
        "declaration".to_string(),
        "law".to_string(),
        "constitution".to_string(),
        "parliament".to_string(),
        "election".to_string(),
        "vote".to_string(),
        "president".to_string(),
        "monarch".to_string(),
        "king".to_string(),
        "queen".to_string(),
        "emperor".to_string(),
        "dynasty".to_string(),
        "civilization".to_string(),
        "culture".to_string(),
        "religion".to_string(),
        "church".to_string(),
        "temple".to_string(),
        "mosque".to_string(),
        "festival".to_string(),
        "holiday".to_string(),
        "celebration".to_string(),
        "anniversary".to_string(),
        "memorial".to_string(),
        "monument".to_string(),
        "ruins".to_string(),
        "archaeology".to_string(),
        "exploration".to_string(),
        "discovery".to_string(),
        "invention".to_string(),
        "science".to_string(),
        "technology".to_string(),
        "industry".to_string(),
        "economy".to_string(),
        "trade".to_string(),
        "commerce".to_string(),
        "migration".to_string(),
        "immigration".to_string(),
        "emigration".to_string(),
        "refugee".to_string(),
        "asylum".to_string(),
        "colonization".to_string(),
        "decolonization".to_string(),
        "imperialism".to_string(),
        "capitalism".to_string(),
        "socialism".to_string(),
        "communism".to_string(),
        "fascism".to_string(),
        "democracy".to_string(),
        "dictatorship".to_string(),
        "authoritarian".to_string(),
        "totalitarian".to_string(),
        "human rights".to_string(),
        "freedom".to_string(),
        "justice".to_string(),
        "equality".to_string(),
        "discrimination".to_string(),
        "segregation".to_string(),
        "apartheid".to_string(),
        "racism".to_string(),
        "sexism".to_string(),
        "colonialism".to_string(),
        "slavery".to_string(),
        "abolition".to_string(),
        "reconstruction".to_string(),
        "modernization".to_string(),
        "industrialization".to_string(),
        "urbanization".to_string(),
        "globalization".to_string(),
        "integration".to_string(),
        "separation".to_string(),
        "unification".to_string(),
        "division".to_string(),
    ]
}

pub async fn fetch_wikipedia_keywords() -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
    let fetcher = WikiFetcher::new();
    fetcher.fetch_all_keywords().await
}