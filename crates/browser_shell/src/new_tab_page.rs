//! New tab page interface.

use crate::search_engines::MultiEngineSearchPage;
<<<<<<< HEAD
use crate::ui_skin::UiSkin;
<<<<<<< HEAD
use crate::{settings_system::BrowserSettings, skin_registry::SkinRegistry};
=======
>>>>>>> main
=======
>>>>>>> main

#[derive(Debug, Clone)]
pub struct NewTabPage {
    pub title: String,
    pub home_url: String,
    pub search: MultiEngineSearchPage,
<<<<<<< HEAD
    pub skin: UiSkin,
=======
>>>>>>> main
}

impl NewTabPage {
    pub fn new(query: impl Into<String>) -> Self {
        Self {
            title: "NUST New Tab".to_string(),
            home_url: "nust://home".to_string(),
            search: MultiEngineSearchPage::innovative_default(query),
<<<<<<< HEAD
            skin: UiSkin::default(),
=======
>>>>>>> main
        }
    }

    pub fn render_html(&self) -> String {
        format!(
<<<<<<< HEAD
            "<style>{}</style>\n<main id=\"new-tab\" class=\"shell-root\">\n  <section class=\"card\" style=\"max-width:1020px;width:100%;padding:24px;\">\n    <header style=\"display:flex;justify-content:space-between;align-items:center;gap:12px;\">\n      <div><p style=\"margin:0;color:#1e3a8a;font-weight:700;\">{}</p><h1 class=\"surface-title\">{}</h1></div>\n      <nav><a id=\"home-option\" href=\"{}\" class=\"chip\" style=\"text-decoration:none;\">Home</a> <button id=\"new-tab-option\" class=\"button\">New Tab</button></nav>\n    </header>\n    <div style=\"margin-top:16px;\">{}\n    </div>\n  </section>\n</main>",
            self.skin.base_css(),
            self.skin.brand,
=======
            "<main id=\"new-tab\">\n  <header>\n    <h1>{}</h1>\n    <nav><a id=\"home-option\" href=\"{}\">Home</a> <button id=\"new-tab-option\">New Tab</button></nav>\n  </header>\n  {}\n</main>",
>>>>>>> main
            self.title,
            self.home_url,
            self.search.render_one_pager_html()
        )
    }
<<<<<<< HEAD

    pub fn with_settings(query: impl Into<String>, settings: &BrowserSettings) -> Self {
        let mut page = Self::new(query);
        page.skin = SkinRegistry::resolve(&settings.skin_mode);
        page
    }
=======
>>>>>>> main
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
<<<<<<< HEAD
<<<<<<< HEAD
        assert!(html.contains("NUST Fusion"));
=======
        assert!(html.contains("NUST Aurora"));
>>>>>>> main
=======
>>>>>>> main
    }
}
