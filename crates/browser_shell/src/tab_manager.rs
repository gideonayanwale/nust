//! Tab management for browser shell.

use crate::new_tab_page::NewTabPage;

#[derive(Debug, Clone)]
pub struct BrowserTab {
    pub id: usize,
    pub title: String,
    pub url: String,
    pub ui_html: String,
    pub pinned: bool,
    pub muted: bool,
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
            pinned: false,
            muted: false,
        };
        self.next_id += 1;
        self.tabs.push(tab.clone());
        tab
    }

    pub fn pin_tab(&mut self, id: usize) {
        if let Some(tab) = self.tabs.iter_mut().find(|tab| tab.id == id) {
            tab.pinned = true;
        }
    }

    pub fn mute_tab(&mut self, id: usize) {
        if let Some(tab) = self.tabs.iter_mut().find(|tab| tab.id == id) {
            tab.muted = true;
        }
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

    #[test]
    fn can_pin_and_mute_tab() {
        let mut manager = TabManagerService::default();
        let tab = manager.open_new_tab("media");
        manager.pin_tab(tab.id);
        manager.mute_tab(tab.id);
        let updated = &manager.tabs()[0];
        assert!(updated.pinned);
        assert!(updated.muted);
    }
}
