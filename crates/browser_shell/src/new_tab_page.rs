//! New tab page interface.

use crate::search_engines::MultiEngineSearchPage;
use crate::ui_skin::UiSkin;
use crate::{
    bookmark_manager::BookmarkManagerService,
    bookmark_tab::BookmarkTab,
    settings_system::{BookmarkTabPosition, BrowserSettings},
    skin_registry::SkinRegistry,
};

#[derive(Debug, Clone)]
pub struct NewTabPage {
    pub title: String,
    pub home_url: String,
    pub search: MultiEngineSearchPage,
    pub bookmarks: BookmarkManagerService,
    pub bookmark_position: BookmarkTabPosition,
    pub skin: UiSkin,
}

impl NewTabPage {
    pub fn new(query: impl Into<String>) -> Self {
        Self {
            title: "NUST New Tab".to_string(),
            home_url: "nust://home".to_string(),
            search: MultiEngineSearchPage::innovative_default(query),
            bookmarks: default_bookmarks(),
            bookmark_position: BookmarkTabPosition::Top,
            skin: UiSkin::default(),
        }
    }

    pub fn render_html(&self) -> String {
        let bookmark_tab = if self.bookmarks.all().is_empty() {
            String::new()
        } else {
            BookmarkTab::render(&self.bookmark_position, &self.bookmarks)
        };
        format!(
            "<style>{}</style>\n<main id=\"new-tab\" class=\"shell-root\">\n  <section class=\"card\" style=\"max-width:1020px;width:100%;padding:24px;\">\n    <header style=\"display:flex;justify-content:space-between;align-items:center;gap:12px;\">\n      <div><p style=\"margin:0;color:#1e3a8a;font-weight:700;\">{}</p><h1 class=\"surface-title\">{}</h1></div>\n      <nav><a id=\"home-option\" href=\"{}\" class=\"chip\" style=\"text-decoration:none;\">Home</a> <button id=\"new-tab-option\" class=\"button\">New Tab</button></nav>\n    </header>\n    {}\n    <div style=\"margin-top:16px;\">{}\n    </div>\n  </section>\n</main>",
            self.skin.base_css(),
            self.skin.brand,
            self.title,
            self.home_url,
            bookmark_tab,
            self.search.render_one_pager_html()
        )
    }

    pub fn with_settings(query: impl Into<String>, settings: &BrowserSettings) -> Self {
        let mut page = Self::new(query);
        page.skin = SkinRegistry::resolve(&settings.skin_mode);
        page.bookmark_position = settings.desktop_bookmark_tab_position.clone();
        if !settings.desktop_bookmark_tab_enabled {
            page.bookmarks = BookmarkManagerService::default();
        }
        page
    }
}

fn default_bookmarks() -> BookmarkManagerService {
    let mut bookmarks = BookmarkManagerService::default();
    bookmarks.add(
        "https://inbox.example.com",
        "Inbox",
        Some("Desktop".to_string()),
    );
    bookmarks.add(
        "https://calendar.example.com",
        "Calendar",
        Some("Desktop".to_string()),
    );
    bookmarks
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
        assert!(html.contains("NUST Fusion"));
        assert!(html.contains("bookmark-tab"));
    }
}
