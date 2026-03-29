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
        let q = percent_encode_query_component(query.trim());
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
        let escaped_query = escape_html_attribute(&self.query);
        let cards = self
            .planned_requests()
            .iter()
            .map(|request| {
                format!(
                    "<li style=\"list-style:none;margin:10px 0;\"><a href=\"{}\" target=\"_blank\" style=\"display:flex;justify-content:space-between;padding:12px 14px;border-radius:12px;background:white;border:1px solid #d0d7de;text-decoration:none;color:#0f172a;\"><span>{}</span><span style=\"color:#2563eb;\">Open ↗</span></a></li>",
                    request.url,
                    request.engine.label()
                )
            })
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            "<section id=\"multi-engine-search\">\n  <h2 style=\"margin:0 0 10px 0;\">Search everywhere (one page)</h2>\n  <form style=\"display:flex;gap:10px;margin-bottom:16px;\">\n    <input id=\"search-input\" name=\"q\" value=\"{}\" placeholder=\"Search across engines\" style=\"flex:1;padding:12px;border-radius:10px;border:1px solid #9aa4b2;\" />\n    <button type=\"submit\" class=\"button\">Search</button>\n  </form>\n  <ul style=\"padding:0;margin:0;\">\n{}\n  </ul>\n</section>",
            escaped_query, cards
        )
    }
}

fn percent_encode_query_component(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    for &byte in input.as_bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'~') {
            output.push(byte as char);
        } else if byte == b' ' {
            output.push('+');
        } else {
            output.push('%');
            output.push_str(&format!("{byte:02X}"));
        }
    }
    output
}

fn escape_html_attribute(input: &str) -> String {
    let mut escaped = String::with_capacity(input.len());
    for ch in input.chars() {
        match ch {
            '&' => escaped.push_str("&amp;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#x27;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            _ => escaped.push(ch),
        }
    }
    escaped
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

    #[test]
    fn encodes_reserved_query_characters_for_urls() {
        let query = "a&b=#\"test";
        let page = MultiEngineSearchPage::innovative_default(query);
        let encoded = page
            .planned_requests()
            .into_iter()
            .find(|r| r.engine == SearchEngine::Google)
            .expect("expected Google request")
            .url;

        assert!(encoded.ends_with("q=a%26b%3D%23%22test"));
    }

    #[test]
    fn escapes_query_when_rendering_search_input() {
        let page = MultiEngineSearchPage::innovative_default("x\" autofocus");
        let html = page.render_one_pager_html();

        assert!(html.contains("value=\"x&quot; autofocus\""));
        assert!(!html.contains("value=\"x\" autofocus\""));
    }
}
