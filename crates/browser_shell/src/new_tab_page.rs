//! New tab page interface.

use crate::search_engines::MultiEngineSearchPage;

#[derive(Debug, Clone)]
pub struct NewTabPage {
    pub title: String,
    pub home_url: String,
    pub search: MultiEngineSearchPage,
}

impl NewTabPage {
    pub fn new(query: impl Into<String>) -> Self {
        Self {
            title: "NUST New Tab".to_string(),
            home_url: "nust://home".to_string(),
            search: MultiEngineSearchPage::innovative_default(query),
        }
    }

    pub fn render_html(&self) -> String {
        format!(
            "<main id=\"new-tab\">\n  <header>\n    <h1>{}</h1>\n    <nav><a id=\"home-option\" href=\"{}\">Home</a> <button id=\"new-tab-option\">New Tab</button></nav>\n  </header>\n  {}\n</main>",
            self.title,
            self.home_url,
            self.search.render_one_pager_html()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::NewTabPage;

    #[test]
    fn new_tab_layout_is_search_ready() {
        let page = NewTabPage::new("distributed scraping");
        let html = page.render_html();
        assert!(html.contains("id=\"new-tab\""));
        assert!(html.contains("home-option"));
        assert!(html.contains("search-input"));
    }
}
