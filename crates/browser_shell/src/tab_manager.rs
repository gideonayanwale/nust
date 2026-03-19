//! Tab management for browser shell.

use crate::new_tab_page::NewTabPage;
<<<<<<< HEAD
use crate::tab_process_registry::TabProcessInfo;
=======
>>>>>>> main

#[derive(Debug, Clone)]
pub struct BrowserTab {
    pub id: usize,
    pub title: String,
    pub url: String,
    pub ui_html: String,
<<<<<<< HEAD
    pub pinned: bool,
    pub muted: bool,
<<<<<<< HEAD
    pub process: Option<TabProcessInfo>,
=======
>>>>>>> main
=======
>>>>>>> main
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
<<<<<<< HEAD
            pinned: false,
            muted: false,
<<<<<<< HEAD
            process: None,
=======
>>>>>>> main
=======
>>>>>>> main
        };
        self.next_id += 1;
        self.tabs.push(tab.clone());
        tab
    }

<<<<<<< HEAD
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

<<<<<<< HEAD
=======
=======
>>>>>>> main
>>>>>>> main
    pub fn tabs(&self) -> &[BrowserTab] {
        &self.tabs
    }

    pub fn attach_process(&mut self, id: usize, process: TabProcessInfo) {
        if let Some(tab) = self.tabs.iter_mut().find(|tab| tab.id == id) {
            tab.process = Some(process);
        }
    }
=======
    pub fn tabs(&self) -> &[BrowserTab] {
        &self.tabs
    }
>>>>>>> main
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
<<<<<<< HEAD

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
<<<<<<< HEAD

    #[test]
    fn can_attach_process_record_to_tab() {
        let mut manager = TabManagerService::default();
        let tab = manager.open_new_tab("automation");
        let mut registry = crate::tab_process_registry::TabProcessRegistry::default();
        let process = registry.register_thread_process_for_tab(tab.id);
        manager.attach_process(tab.id, process);
        assert!(manager.tabs()[0].process.is_some());
    }
=======
>>>>>>> main
}
=======
}
//! Tab Manager module scaffold.

#[derive(Debug, Default)]
pub struct TabManagerService;
>>>>>>> main
