//! Home page model for the browser shell.

use crate::ui_skin::UiSkin;
use crate::{
    bookmark_manager::BookmarkManagerService,
    bookmark_tab::BookmarkTab,
    settings_system::{BookmarkTabPosition, BrowserSettings},
    skin_registry::SkinRegistry,
};

#[derive(Debug, Clone)]
pub struct HomePage {
    pub welcome: String,
    pub shortcuts: Vec<&'static str>,
    pub pinned_apps: Vec<String>,
    pub bookmarks: BookmarkManagerService,
    pub bookmark_position: BookmarkTabPosition,
    pub skin: UiSkin,
}

impl Default for HomePage {
    fn default() -> Self {
        Self {
            welcome: "Welcome to NUST Browser OS".to_string(),
            shortcuts: vec!["New Tab", "Automation", "Workspace", "History"],
            pinned_apps: default_pinned_apps(),
            bookmarks: default_bookmarks(),
            bookmark_position: BookmarkTabPosition::Top,
            skin: UiSkin::default(),
        }
    }
}

impl HomePage {
    pub fn with_settings(settings: &BrowserSettings) -> Self {
        let mut page = Self::default();
        page.skin = SkinRegistry::resolve(&settings.skin_mode);
        page.bookmark_position = settings.desktop_bookmark_tab_position.clone();
        if !settings.desktop_bookmark_tab_enabled {
            page.bookmarks = BookmarkManagerService::default();
        }
        page
    }

    pub fn render_html(&self) -> String {
        let shortcuts = self
            .shortcuts
            .iter()
            .map(|shortcut| format!("<span class=\"chip\">{shortcut}</span>"))
            .collect::<Vec<_>>()
            .join("\n      ");

        let pinned_apps = self
            .pinned_apps
            .iter()
            .take(64)
            .map(|app| format!("<div class=\"chip\" style=\"text-align:center;\">{app}</div>"))
            .collect::<Vec<_>>()
            .join("\n      ");

        let bookmark_tab = if self.bookmarks.all().is_empty() {
            String::new()
        } else {
            BookmarkTab::render(&self.bookmark_position, &self.bookmarks)
        };

        format!(
            "<style>{}</style>\n<main class=\"shell-root\">\n  <section id=\"home\" class=\"card\" style=\"max-width:1020px;width:100%;padding:28px;\">\n    <p style=\"margin:0 0 8px 0;font-weight:700;color:#1e3a8a;\">{}</p>\n    <h1 class=\"surface-title\">{}</h1>\n    <p class=\"surface-subtitle\" style=\"margin:10px 0 20px 0;\">Programmable browsing surface with automation-native architecture.</p>\n    {}\n    <div style=\"display:flex;gap:10px;flex-wrap:wrap;\">\n      {}\n    </div>\n    <h2 class=\"surface-subtitle\" style=\"margin-top:18px;font-size:15px;\">Pinned Apps (8 per row, up to 64)</h2>\n    <div id=\"pinned-apps-grid\" style=\"display:grid;grid-template-columns:repeat(8,minmax(0,1fr));gap:8px;\">\n      {}\n    </div>\n  </section>\n</main>",
            self.skin.base_css(), self.skin.brand, self.welcome, bookmark_tab, shortcuts, pinned_apps
        )
    }
}

fn default_pinned_apps() -> Vec<String> {
    vec![
        "Mail",
        "Docs",
        "Calendar",
        "Tasks",
        "Chat",
        "Drive",
        "Sheets",
        "Slides",
        "Git",
        "CI/CD",
        "Logs",
        "Analytics",
        "CRM",
        "ERP",
        "Wiki",
        "Design",
    ]
    .into_iter()
    .map(str::to_string)
    .collect()
}

fn default_bookmarks() -> BookmarkManagerService {
    let mut bookmarks = BookmarkManagerService::default();
    bookmarks.add(
        "https://mail.example.com",
        "Mail",
        Some("Desktop".to_string()),
    );
    bookmarks.add(
        "https://docs.example.com",
        "Docs",
        Some("Desktop".to_string()),
    );
    bookmarks.add(
        "https://chat.example.com",
        "Chat",
        Some("Desktop".to_string()),
    );
    bookmarks
}

#[cfg(test)]
mod tests {
    use super::HomePage;

    #[test]
    fn home_page_renders_shortcuts() {
        let html = HomePage::default().render_html();
        assert!(html.contains("Welcome to NUST Browser OS"));
        assert!(html.contains("Automation"));
        assert!(html.contains("NUST Fusion"));
        assert!(html.contains("pinned-apps-grid"));
        assert!(html.contains("bookmark-tab"));
    }

    #[test]
    fn pinned_apps_are_capped_in_render() {
        let mut page = HomePage::default();
        page.pinned_apps = (0..80).map(|i| format!("App-{i}")).collect();
        let html = page.render_html();
        assert!(!html.contains("App-79"));
    }
}
