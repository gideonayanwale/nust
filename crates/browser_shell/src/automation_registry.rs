//! Browser-shell automation registry and default workflows.

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum AutomationCommand {
    Open(String),
    Click(String),
    Type { selector: String, text: String },
    Submit(String),
    WaitFor(String),
}

#[derive(Debug, Default)]
pub struct AutomationRegistry {
    workflows_by_tab: HashMap<usize, Vec<AutomationCommand>>,
}

impl AutomationRegistry {
    pub fn register_default_workflow(&mut self, tab_id: usize, url: &str) {
        self.workflows_by_tab.insert(
            tab_id,
            vec![
                AutomationCommand::Open(url.to_string()),
                AutomationCommand::WaitFor("body".to_string()),
                AutomationCommand::Click("#search-input".to_string()),
                AutomationCommand::Type {
                    selector: "#search-input".to_string(),
                    text: "nust autonomous browsing".to_string(),
                },
                AutomationCommand::Submit("#search-form".to_string()),
            ],
        );
    }

    pub fn workflow_count(&self) -> usize {
        self.workflows_by_tab.len()
    }

    pub fn workflow_for_tab(&self, tab_id: usize) -> Option<&[AutomationCommand]> {
        self.workflows_by_tab.get(&tab_id).map(Vec::as_slice)
    }
}

#[cfg(test)]
mod tests {
    use super::AutomationRegistry;

    #[test]
    fn default_workflow_is_registered_per_tab() {
        let mut registry = AutomationRegistry::default();
        registry.register_default_workflow(7, "https://example.com");
        assert_eq!(registry.workflow_count(), 1);
        assert!(registry.workflow_for_tab(7).is_some());
    }
}
