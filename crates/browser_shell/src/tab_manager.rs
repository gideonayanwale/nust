//! Tab management for browser shell.

use crate::new_tab_page::NewTabPage;
use crate::tab_process_registry::TabProcessInfo;

#[derive(Debug, Clone)]
pub struct BrowserTab {
    pub id: usize,
    pub title: String,
    pub url: String,
    pub ui_html: String,
    pub pinned: bool,
    pub muted: bool,
    pub has_home_button: bool,
    pub process: Option<TabProcessInfo>,
}

#[derive(Debug, Default)]
pub struct TabManagerService {
    tabs: Vec<BrowserTab>,
    split_groups: Vec<(usize, usize)>,
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
            has_home_button: true,
            process: None,
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

    pub fn open_split_screen(&mut self, left_id: usize, right_id: usize) {
        let left_exists = self.tabs.iter().any(|tab| tab.id == left_id);
        let right_exists = self.tabs.iter().any(|tab| tab.id == right_id);
        if left_exists && right_exists && left_id != right_id {
            self.split_groups.push((left_id, right_id));
        }
    }

    pub fn split_groups(&self) -> &[(usize, usize)] {
        &self.split_groups
    }

    pub fn attach_process(&mut self, id: usize, process: TabProcessInfo) {
        if let Some(tab) = self.tabs.iter_mut().find(|tab| tab.id == id) {
            tab.process = Some(process);
        }
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
        assert!(updated.has_home_button);
    }

    #[test]
    fn can_attach_process_record_to_tab() {
        let mut manager = TabManagerService::default();
        let tab = manager.open_new_tab("automation");
        let mut registry = crate::tab_process_registry::TabProcessRegistry::default();
        let process = registry.register_thread_process_for_tab(tab.id);
        manager.attach_process(tab.id, process);
        assert!(manager.tabs()[0].process.is_some());
    }

    #[test]
    fn can_create_split_screen_group() {
        let mut manager = TabManagerService::default();
        let a = manager.open_new_tab("left");
        let b = manager.open_new_tab("right");
        manager.open_split_screen(a.id, b.id);
        assert_eq!(manager.split_groups().len(), 1);
    }
}
