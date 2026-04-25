use std::io::stdout;
use std::sync::atomic::{AtomicBool, Ordering};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Terminal, Frame,
};
use chrono::Local;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::config::{Config, CATEGORIES};
use crate::rss::NewsItem;
use crate::wiki::WikiAnniversary;
use crate::notify::NotificationManager;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

pub async fn run_app(
    config: Config,
    rss_fetcher: Arc<Mutex<crate::rss::RssFetcher>>,
    wiki_fetcher: Arc<Mutex<crate::wiki::WikiFetcher>>,
    _notifier: NotificationManager,
) -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    let running = Arc::new(AtomicBool::new(true));
    
    let mut news_by_category: std::collections::HashMap<String, Vec<NewsItem>> = std::collections::HashMap::new();
    let mut anniversaries: Vec<WikiAnniversary> = Vec::new();
    let scan_interval = Duration::from_secs(config.scan_interval_secs);
    
    let mut selected_category: usize = 0;
    let mut selected_news_index: usize = 0;
    let mut view_mode: ViewMode = ViewMode::List;
    
    let mut last_fetch = std::time::Instant::now() - scan_interval;
    
    while running.load(Ordering::Relaxed) {
        if last_fetch.elapsed() >= scan_interval {
            last_fetch = std::time::Instant::now();
            
            let feeds: Vec<(&str, &str, &str)> = config.rss_feeds.iter()
                .map(|f| (f.url.as_str(), f.name.as_str(), f.category.as_str()))
                .collect();
            
            let fetcher = rss_fetcher.lock().await;
            
            match fetcher.fetch_multiple(&feeds).await {
                Ok(items) => {
                    println!("[RSS] Fetched {} items", items.len());
                    news_by_category.clear();
                    for item in items {
                        news_by_category
                            .entry(item.category.clone())
                            .or_insert_with(Vec::new)
                            .push(item);
                    }
                },
                Err(e) => println!("[RSS] Error: {}", e),
            }
            
            let wiki = wiki_fetcher.lock().await;
            match wiki.fetch_today_anniversaries().await {
                Ok(items) => anniversaries = items,
                Err(e) => println!("[Wiki] error: {}", e),
            }
        }
        
        if view_mode == ViewMode::List {
            terminal.draw(|f| {
                render_list_view(f, &config, &news_by_category, &anniversaries, selected_category, selected_news_index);
            })?;
        } else {
            let current_category = CATEGORIES[selected_category];
            if let Some(news_items) = news_by_category.get(current_category) {
                if let Some(item) = news_items.get(selected_news_index) {
                    terminal.draw(|f| {
                        render_detail_view(f, item);
                    })?;
                }
            }
        }
        
        if event::poll(Duration::from_millis(30)).unwrap_or(false) {
            if let Ok(Event::Key(key)) = event::read() {
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                
                match key.code {
                    KeyCode::Char('q') => {
                        running.store(false, Ordering::Relaxed);
                    }
                    KeyCode::Tab => {
                        if view_mode == ViewMode::List {
                            selected_category = (selected_category + 1) % CATEGORIES.len();
                            selected_news_index = 0;
                        }
                    }
                    KeyCode::BackTab => {
                        if view_mode == ViewMode::List {
                            selected_category = if selected_category == 0 {
                                CATEGORIES.len() - 1
                            } else {
                                selected_category - 1
                            };
                            selected_news_index = 0;
                        }
                    }
                    KeyCode::Enter => {
                        if view_mode == ViewMode::List {
                            let current_category = CATEGORIES[selected_category];
                            if let Some(news_items) = news_by_category.get(current_category) {
                                if !news_items.is_empty() && selected_news_index < news_items.len() {
                                    view_mode = ViewMode::Detail;
                                }
                            }
                        }
                    }
                    KeyCode::Esc => {
                        view_mode = ViewMode::List;
                    }
                    KeyCode::Up => {
                        if view_mode == ViewMode::List && selected_news_index > 0 {
                            selected_news_index -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if view_mode == ViewMode::List {
                            let current_category = CATEGORIES[selected_category];
                            if let Some(news_items) = news_by_category.get(current_category) {
                                if selected_news_index + 1 < news_items.len() {
                                    selected_news_index += 1;
                                }
                            }
                        }
                    }
                    KeyCode::Char('r') => {
                        last_fetch = std::time::Instant::now() - scan_interval;
                    }
                    KeyCode::Char('h') => { selected_category = 0; selected_news_index = 0; view_mode = ViewMode::List; }
                    KeyCode::Char('w') => { selected_category = 1; selected_news_index = 0; view_mode = ViewMode::List; }
                    KeyCode::Char('s') => { selected_category = 2; selected_news_index = 0; view_mode = ViewMode::List; }
                    KeyCode::Char('p') => { selected_category = 3; selected_news_index = 0; view_mode = ViewMode::List; }
                    KeyCode::Char('e') => { selected_category = 4; selected_news_index = 0; view_mode = ViewMode::List; }
                    KeyCode::Char('t') => { selected_category = 5; selected_news_index = 0; view_mode = ViewMode::List; }
                    KeyCode::Char('c') => { selected_category = 6; selected_news_index = 0; view_mode = ViewMode::List; }
                    KeyCode::Char('a') => { selected_category = 7; selected_news_index = 0; view_mode = ViewMode::List; }
                    KeyCode::Char('b') => { selected_category = 8; selected_news_index = 0; view_mode = ViewMode::List; }
                    KeyCode::Char('g') => { selected_category = 9; selected_news_index = 0; view_mode = ViewMode::List; }
                    KeyCode::Char(c) if c.is_ascii_digit() => {
                        if let Some(digit) = c.to_digit(10) {
                            let idx = if digit == 0 { 9 } else { (digit - 1) as usize };
                            if idx < CATEGORIES.len() {
                                selected_category = idx;
                                selected_news_index = 0;
                                view_mode = ViewMode::List;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        
        std::thread::sleep(Duration::from_millis(30));
    }
    
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;
    
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ViewMode {
    List,
    Detail,
}

fn render_list_view(
    f: &mut Frame,
    config: &Config,
    news_by_category: &std::collections::HashMap<String, Vec<NewsItem>>,
    anniversaries: &[WikiAnniversary],
    selected_category: usize,
    selected_news_index: usize,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(f.area());
    
    let header = format!(
        " KARGA NEWS {} | [Tab] Sonraki | [Enter] Detay | h=Haber w=Dunya s=Spor p=Politika t=Tekno q=Cikis ",
        Local::now().format("%H:%M")
    );
    
    let header_block = Block::default()
        .style(Style::default().bg(parse_color("#0a1628")))
        .title(header)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(parse_color("#4a90d9")));
    
    f.render_widget(header_block, chunks[0]);
    
    let tab_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            CATEGORIES.iter()
                .map(|_| Constraint::Ratio(1, CATEGORIES.len() as u32))
                .collect::<Vec<_>>()
        )
        .split(chunks[1]);
    
    for (i, cat) in CATEGORIES.iter().enumerate() {
        let is_selected = i == selected_category;
        let style = if is_selected {
            Style::default().fg(parse_color("#4a90d9")).bg(parse_color("#2a3a4a"))
        } else {
            Style::default().fg(Color::Gray)
        };
        
        let label = if is_selected { format!("[{}]", cat) } else { format!(" {} ", cat) };
        f.render_widget(Paragraph::new(label).style(style), tab_chunks[i]);
    }
    
    let current_category = CATEGORIES[selected_category];
    let news_items = news_by_category.get(current_category).cloned().unwrap_or_default();
    
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(70),
            Constraint::Percentage(30),
        ])
        .split(chunks[2]);
    
    let list_block = Block::default()
        .style(Style::default().bg(parse_color("#1c2128")))
        .title(format!(" {} ({}) ", current_category, news_items.len()))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Black));
    
    f.render_widget(list_block, main_chunks[0]);
    
    let mut extra_items: Vec<(String, String)> = Vec::new();
    if current_category == "Dünya" {
        if let Some(tech) = news_by_category.get("Teknoloji") {
            for item in tech.iter().take(1) {
                extra_items.push(("Teknoloji".to_string(), item.title.clone()));
            }
        }
        if let Some(ekon) = news_by_category.get("Ekonomi") {
            for item in ekon.iter().take(1) {
                extra_items.push(("Ekonomi".to_string(), item.title.clone()));
            }
        }
    }
    
    let inner = main_chunks[0].inner(ratatui::layout::Margin { horizontal: 1, vertical: 1 });
    let mut items: Vec<ListItem> = Vec::new();
    
    for (i, item) in news_items.iter().enumerate() {
        let matched = matches_keywords(&item.title, &config.keywords);
        let prefix = if matched { "[+] " } else { "- " };
        let content = format!("{}{}", prefix, safe_truncate(&item.title, 80));
        
        let style = if i == selected_news_index {
            Style::default().fg(parse_color("#4a90d9")).bg(parse_color("#1a2a3a"))
        } else {
            Style::default().fg(Color::White)
        };
        
        items.push(ListItem::new(content).style(style));
    }
    
    if items.is_empty() {
        items.push(ListItem::new("Haber yok").style(Style::default().fg(Color::DarkGray)));
    }
    
    f.render_widget(List::new(items).block(Block::default()), inner);
    
    if !extra_items.is_empty() {
        let extra_block = Block::default()
            .style(Style::default().bg(parse_color("#0d1117")))
            .title(" Diğer Kategoriler ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));
        f.render_widget(extra_block, main_chunks[1]);
        
        let extra_inner = main_chunks[1].inner(ratatui::layout::Margin { horizontal: 1, vertical: 1 });
        let extra_list: Vec<ListItem> = extra_items.iter()
            .map(|(cat, title)| {
                ListItem::new(format!("{}: {}", cat, safe_truncate(title, 40)))
                    .style(Style::default().fg(Color::Gray))
            })
            .collect();
        f.render_widget(List::new(extra_list).block(Block::default()), extra_inner);
} else {
        let history_block = Block::default()
            .style(Style::default().bg(parse_color("#0d1117")))
            .title(" Tarihte Bugun ")
            .borders(Borders::ALL);
        
        f.render_widget(history_block, main_chunks[1]);
        
        let history_inner = main_chunks[1].inner(ratatui::layout::Margin { horizontal: 1, vertical: 1 });
        let mut history_items: Vec<ListItem> = Vec::new();
        
        for ann in anniversaries.iter().take(8) {
            history_items.push(ListItem::new(format!("[{}] {}", ann.year, safe_truncate(&ann.description, 40))));
        }
        
        if history_items.is_empty() {
            history_items.push(ListItem::new("Veri yok").style(Style::default().fg(Color::DarkGray)));
        }
        
        f.render_widget(List::new(history_items).block(Block::default()), history_inner);
    }
}
    

fn render_detail_view(f: &mut Frame, item: &NewsItem) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(f.area());
    
    let header = format!(" [Esc] GERI | {} ", safe_truncate(&item.title, 60));
    let header_block = Block::default()
        .style(Style::default().bg(parse_color("#0a1628")))
        .title(header)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(parse_color("#4a90d9")));
    
    f.render_widget(header_block, chunks[0]);
    
    let content_block = Block::default()
        .style(Style::default().bg(parse_color("#1c2128")))
        .borders(Borders::ALL);
    
    f.render_widget(content_block, chunks[1]);
    
    let inner = chunks[1].inner(ratatui::layout::Margin { horizontal: 1, vertical: 1 });
    
    let title = Paragraph::new(safe_truncate(&item.title, 50))
        .style(Style::default().fg(parse_color("#4a90d9")).bg(parse_color("#1c2128")));
    
    let date = if item.pub_date.chars().count() > 16 {
        item.pub_date.chars().take(16).collect::<String>()
    } else {
        item.pub_date.clone()
    };
    let meta = Paragraph::new(format!("{} | {} | {}", item.source, date, item.category))
        .style(Style::default().fg(Color::Gray).bg(parse_color("#1c2128")));
    
    let divider = Paragraph::new("----------------------")
        .style(Style::default().fg(Color::DarkGray).bg(parse_color("#1c2128")));
    
    let clean_desc = clean_html(&item.description);
    let wrapped_desc = wrap_text(&clean_desc, inner.width as usize);
    let desc = Paragraph::new(wrapped_desc)
        .style(Style::default().fg(Color::White).bg(parse_color("#1c2128")))
        .wrap(Wrap { trim: true });
    
    let link = Paragraph::new(item.link.clone())
        .style(Style::default().fg(Color::Blue).bg(parse_color("#1c2128")));
    
    let blocks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(2),
        ])
        .split(inner);
    
    f.render_widget(title, blocks[0]);
    f.render_widget(meta, blocks[1]);
    f.render_widget(divider, blocks[2]);
    f.render_widget(desc, blocks[3]);
    f.render_widget(link, blocks[4]);
}

fn safe_truncate(s: &str, max_chars: usize) -> String {
    let char_count = s.chars().count();
    if char_count <= max_chars {
        return s.to_string();
    }
    let truncated: String = s.chars().take(max_chars.saturating_sub(3)).collect();
    format!("{}...", truncated)
}

fn wrap_text(text: &str, width: usize) -> String {
    if text.chars().count() <= width {
        return text.to_string();
    }
    
    let mut result = String::new();
    let mut current_len = 0;
    
    for word in text.split_whitespace() {
        let word_len = word.chars().count();
        if current_len + word_len + 1 > width {
            result.push('\n');
            result.push_str(word);
            current_len = word_len;
        } else {
            if current_len > 0 {
                result.push(' ');
                current_len += 1;
            }
            result.push_str(word);
            current_len += word_len;
        }
    }
    
    result
}

fn matches_keywords(title: &str, keywords: &[String]) -> bool {
    let title_lower = title.to_lowercase();
    for keyword in keywords.iter().take(100) {
        if title_lower.contains(&keyword.to_lowercase()) {
            return true;
        }
    }
    false
}

fn parse_color(hex: &str) -> Color {
    if hex.starts_with('#') && hex.len() == 7 {
        let r = u8::from_str_radix(&hex[1..3], 16).unwrap_or(0);
        let g = u8::from_str_radix(&hex[3..5], 16).unwrap_or(0);
        let b = u8::from_str_radix(&hex[5..7], 16).unwrap_or(0);
        Color::Rgb(r, g, b)
    } else {
        Color::White
    }
}

fn clean_html(text: &str) -> String {
    let mut result = String::new();
    let mut in_tag = false;
    let mut in_para = false;
    
    for ch in text.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => {
                in_tag = false;
                if result.ends_with("p>") {
                    result.push('\n');
                    result.push('\n');
                    in_para = true;
                }
            }
            '\n' | '\r' => continue,
            _ if in_tag => continue,
            _ => {
                if in_para && ch.is_whitespace() {
                    continue;
                }
                in_para = false;
                result.push(ch);
            }
        }
    }
    
    let re = regex::Regex::new(r"\s+").unwrap();
    re.replace_all(&result, " ").trim().to_string()
}