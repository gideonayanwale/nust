//! Tab suggestions and AI insights.

use crate::history_manager::HistoryManagerService;

#[derive(Debug, Clone)]
pub struct TabInsight {
    pub suggestion: String,
    pub ai_insight: String,
}

#[derive(Debug, Default)]
pub struct TabIntelligenceService;

impl TabIntelligenceService {
    pub fn suggest_for_query(
        &self,
        query: &str,
        history: &HistoryManagerService,
    ) -> Vec<TabInsight> {
        let history_hits = history
            .search(query)
            .into_iter()
            .take(3)
            .collect::<Vec<_>>();
        if history_hits.is_empty() {
            return vec![TabInsight {
                suggestion: format!("Open a focused research tab for '{query}'"),
                ai_insight: "No strong history pattern yet; start broad and refine.".to_string(),
            }];
        }

        history_hits
            .into_iter()
            .map(|entry| TabInsight {
                suggestion: format!("Reopen: {}", entry.title),
                ai_insight: format!("High relevance based on prior visit to {}", entry.url),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::history_manager::HistoryManagerService;

    use super::TabIntelligenceService;

    #[test]
    fn provides_ai_insights_from_history() {
        let mut history = HistoryManagerService::default();
        history.visit("https://example.com/rust", "Rust Guide", 1_700_000_000_000);
        let insights = TabIntelligenceService.suggest_for_query("rust", &history);
        assert!(!insights.is_empty());
        assert!(insights[0].ai_insight.contains("relevance"));
    }
}
