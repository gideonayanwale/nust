<<<<<<< HEAD
//! Lightweight history manager (modern browser primitive).

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoryEntry {
    pub url: String,
    pub title: String,
    pub timestamp_unix_ms: u64,
}

#[derive(Debug, Default)]
pub struct HistoryManagerService {
    entries: Vec<HistoryEntry>,
}

impl HistoryManagerService {
    pub fn visit(
        &mut self,
        url: impl Into<String>,
        title: impl Into<String>,
        timestamp_unix_ms: u64,
    ) {
        self.entries.push(HistoryEntry {
            url: url.into(),
            title: title.into(),
            timestamp_unix_ms,
        });
    }

    pub fn recent(&self, limit: usize) -> Vec<HistoryEntry> {
        self.entries.iter().rev().take(limit).cloned().collect()
    }

    pub fn search(&self, query: &str) -> Vec<HistoryEntry> {
        let q = query.to_lowercase();
        self.entries
            .iter()
            .filter(|entry| {
                entry.url.to_lowercase().contains(&q) || entry.title.to_lowercase().contains(&q)
            })
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::HistoryManagerService;

    #[test]
    fn supports_recent_and_search() {
        let mut history = HistoryManagerService::default();
        history.visit("https://example.com", "Example", 1);
        history.visit("https://rust-lang.org", "Rust", 2);

        let recent = history.recent(1);
        assert_eq!(recent.len(), 1);
        assert_eq!(recent[0].title, "Rust");

        let search = history.search("rust");
        assert_eq!(search.len(), 1);
        assert_eq!(search[0].url, "https://rust-lang.org");
    }
}
=======
//! History Manager module scaffold.

#[derive(Debug, Default)]
pub struct HistoryManagerService;
>>>>>>> main
