//! Home page model for the browser shell.

use crate::ui_skin::UiSkin;

#[derive(Debug, Clone)]
pub struct HomePage {
    pub welcome: String,
    pub shortcuts: Vec<&'static str>,
    pub skin: UiSkin,
}

impl Default for HomePage {
    fn default() -> Self {
        Self {
            welcome: "Welcome to NUST Browser OS".to_string(),
            shortcuts: vec!["New Tab", "Automation", "Workspace", "History"],
            skin: UiSkin::default(),
        }
    }
}

impl HomePage {
    pub fn render_html(&self) -> String {
        let shortcuts = self
            .shortcuts
            .iter()
            .map(|shortcut| format!("<span class=\"chip\">{shortcut}</span>"))
            .collect::<Vec<_>>()
            .join("\n      ");

        format!(
            "<style>{}</style>\n<main class=\"shell-root\">\n  <section id=\"home\" class=\"card\" style=\"max-width:920px;width:100%;padding:28px;\">\n    <p style=\"margin:0 0 8px 0;font-weight:700;color:#1e3a8a;\">{}</p>\n    <h1 class=\"surface-title\">{}</h1>\n    <p class=\"surface-subtitle\" style=\"margin:10px 0 20px 0;\">Programmable browsing surface with automation-native architecture.</p>\n    <div style=\"display:flex;gap:10px;flex-wrap:wrap;\">\n      {}\n    </div>\n  </section>\n</main>",
            self.skin.base_css(), self.skin.brand, self.welcome, shortcuts
        )
    }
}

#[cfg(test)]
mod tests {
    use super::HomePage;

    #[test]
    fn home_page_renders_shortcuts() {
        let html = HomePage::default().render_html();
        assert!(html.contains("Welcome to NUST Browser OS"));
        assert!(html.contains("Automation"));
        assert!(html.contains("NUST Aurora"));
    }
}
