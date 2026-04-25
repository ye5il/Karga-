#[allow(dead_code)]
pub struct NotificationManager {
    enabled: bool,
}

#[allow(dead_code)]
impl NotificationManager {
    pub fn new() -> Self {
        Self {
            enabled: true,
        }
    }

    pub fn send(&self, title: &str, body: &str) {
        if self.enabled {
            if let Err(e) = notify_rust::Notification::new()
                .summary(title)
                .body(body)
                .show()
            {
                eprintln!("[Notification] Failed: {}", e);
            }
        }
    }

    pub fn send_new_news(&self, source: &str, count: usize) {
        self.send(
            "Karga News",
            &format!("{} new articles from {}", count, source),
        );
    }
}