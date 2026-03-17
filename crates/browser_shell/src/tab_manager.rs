//! Tab management for browser shell.

use crate::new_tab_page::NewTabPage;

#[derive(Debug, Clone)]
pub struct BrowserTab {
    pub id: usize,
    pub title: String,
    pub url: String,
    pub ui_html: String,
}

#[derive(Debug, Default)]
pub struct TabManagerService {
    tabs: Vec<BrowserTab>,
    next_id: usize,
}

impl TabManagerService {
    pub fn open_new_tab(&mut self, query: impl Into<String>) -> BrowserTab {
        let page = NewTabPage::new(query);
        let tab = BrowserTab {
            id: self.next_id,
            title: page.title.clone(),
            url: "nust://new-tab".to_string(),
            ui_html: page.render_html(),
        };
        self.next_id += 1;
        self.tabs.push(tab.clone());
        tab
    }

    pub fn tabs(&self) -> &[BrowserTab] {
        &self.tabs
    }
}

#[cfg(test)]
mod tests {
    use super::TabManagerService;

    #[test]
    fn opens_new_tab_with_search_interface() {
        let mut manager = TabManagerService::default();
        let tab = manager.open_new_tab("nust ai browser");
        assert_eq!(tab.id, 0);
        assert!(tab.ui_html.contains("Search everywhere"));
        assert_eq!(manager.tabs().len(), 1);
    }
}
