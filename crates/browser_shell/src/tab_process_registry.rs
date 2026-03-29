//! Registry that tracks a lightweight thread-backed process per tab.

use std::collections::HashMap;
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TabProcessState {
    Running,
    Completed,
}

#[derive(Debug, Clone)]
pub struct TabProcessInfo {
    pub tab_id: usize,
    pub thread_name: String,
    pub state: TabProcessState,
}

#[derive(Debug, Default)]
pub struct TabProcessRegistry {
    processes: HashMap<usize, TabProcessInfo>,
}

impl TabProcessRegistry {
    pub fn register_thread_process_for_tab(&mut self, tab_id: usize) -> TabProcessInfo {
        let thread_name = format!("tab-worker-{tab_id}");
        let thread_name_for_spawn = thread_name.clone();
        let _ = thread::Builder::new()
            .name(thread_name_for_spawn)
            .spawn(|| {
                thread::sleep(Duration::from_millis(5));
            });

        let info = TabProcessInfo {
            tab_id,
            thread_name,
            state: TabProcessState::Running,
        };
        self.processes.insert(tab_id, info.clone());
        info
    }

    pub fn mark_completed(&mut self, tab_id: usize) {
        if let Some(process) = self.processes.get_mut(&tab_id) {
            process.state = TabProcessState::Completed;
        }
    }

    pub fn get(&self, tab_id: usize) -> Option<&TabProcessInfo> {
        self.processes.get(&tab_id)
    }
}

#[cfg(test)]
mod tests {
    use super::{TabProcessRegistry, TabProcessState};

    #[test]
    fn can_register_and_complete_tab_process() {
        let mut registry = TabProcessRegistry::default();
        registry.register_thread_process_for_tab(42);
        assert_eq!(
            registry.get(42).map(|t| t.state.clone()),
            Some(TabProcessState::Running)
        );
        registry.mark_completed(42);
        assert_eq!(
            registry.get(42).map(|t| t.state.clone()),
            Some(TabProcessState::Completed)
        );
    }
}
