//! Unified multi-engine search planning and one-page UI rendering.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchEngine {
    Google,
    Bing,
    DuckDuckGo,
    Brave,
    Perplexity,
}

impl SearchEngine {
    pub fn label(self) -> &'static str {
        match self {
            SearchEngine::Google => "Google",
            SearchEngine::Bing => "Bing",
            SearchEngine::DuckDuckGo => "DuckDuckGo",
            SearchEngine::Brave => "Brave",
            SearchEngine::Perplexity => "Perplexity",
        }
    }

    pub fn search_url(self, query: &str) -> String {
        let q = query.trim().replace(' ', "+");
        match self {
            SearchEngine::Google => format!("https://www.google.com/search?q={q}"),
            SearchEngine::Bing => format!("https://www.bing.com/search?q={q}"),
            SearchEngine::DuckDuckGo => format!("https://duckduckgo.com/?q={q}"),
            SearchEngine::Brave => format!("https://search.brave.com/search?q={q}"),
            SearchEngine::Perplexity => format!("https://www.perplexity.ai/search?q={q}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchRequest {
    pub engine: SearchEngine,
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct MultiEngineSearchPage {
    pub query: String,
    pub engines: Vec<SearchEngine>,
}

impl MultiEngineSearchPage {
    pub fn innovative_default(query: impl Into<String>) -> Self {
        Self {
            query: query.into(),
            engines: vec![
                SearchEngine::Google,
                SearchEngine::Bing,
                SearchEngine::DuckDuckGo,
                SearchEngine::Brave,
                SearchEngine::Perplexity,
            ],
        }
    }

    pub fn planned_requests(&self) -> Vec<SearchRequest> {
        self.engines
            .iter()
            .copied()
            .map(|engine| SearchRequest {
                engine,
                url: engine.search_url(&self.query),
            })
            .collect()
    }

    pub fn render_one_pager_html(&self) -> String {
        let cards = self
            .planned_requests()
            .iter()
            .map(|request| {
                format!(
                    "<li><a href=\"{}\" target=\"_blank\">{}</a></li>",
                    request.url,
                    request.engine.label()
                )
            })
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            "<section id=\"multi-engine-search\">\n  <h1>Search everywhere (one page)</h1>\n  <form>\n    <input id=\"search-input\" name=\"q\" value=\"{}\" placeholder=\"Search across engines\" />\n    <button type=\"submit\">Search</button>\n  </form>\n  <ul>\n{}\n  </ul>\n</section>",
            self.query, cards
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{MultiEngineSearchPage, SearchEngine};

    #[test]
    fn builds_search_url_per_engine() {
        let page = MultiEngineSearchPage::innovative_default("rust browser engine");
        let plan = page.planned_requests();
        assert_eq!(plan.len(), 5);
        assert!(plan
            .iter()
            .any(|r| r.engine == SearchEngine::Google && r.url.contains("google")));
        assert!(plan
            .iter()
            .any(|r| r.engine == SearchEngine::DuckDuckGo && r.url.contains("duckduckgo")));
    }

    #[test]
    fn renders_input_and_engine_links() {
        let page = MultiEngineSearchPage::innovative_default("nust");
        let html = page.render_one_pager_html();
        assert!(html.contains("search-input"));
        assert!(html.contains("Search everywhere"));
        assert!(html.contains("Perplexity"));
    }
}
