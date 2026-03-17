//! Home page model for the browser shell.

#[derive(Debug, Clone)]
pub struct HomePage {
    pub welcome: String,
    pub shortcuts: Vec<&'static str>,
}

impl Default for HomePage {
    fn default() -> Self {
        Self {
            welcome: "Welcome to NUST Browser OS".to_string(),
            shortcuts: vec!["New Tab", "Automation", "Workspace", "History"],
        }
    }
}

impl HomePage {
    pub fn render_html(&self) -> String {
        let shortcuts = self
            .shortcuts
            .iter()
            .map(|shortcut| format!("<li>{shortcut}</li>"))
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            "<section id=\"home\">\n  <h1>{}</h1>\n  <ul>{}</ul>\n</section>",
            self.welcome, shortcuts
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
    }
}
