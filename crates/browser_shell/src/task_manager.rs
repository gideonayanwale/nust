//! Browser task manager for tabs, processes, and downloads.

use crate::download_manager::DownloadManagerService;
use crate::settings_system::BrowserSettings;
use crate::tab_manager::TabManagerService;

#[derive(Debug, Clone)]
pub struct TaskManagerSnapshot {
    pub tab_count: usize,
    pub process_count: usize,
    pub download_count: usize,
    pub completed_download_count: usize,
}

#[derive(Debug, Default)]
pub struct TaskManagerService;

impl TaskManagerService {
    pub fn snapshot(
        &self,
        tabs: &TabManagerService,
        downloads: &DownloadManagerService,
        settings: &BrowserSettings,
    ) -> Option<TaskManagerSnapshot> {
        if !settings.task_manager_enabled {
            return None;
        }

        let tab_count = tabs.tabs().len();
        let process_count = tabs
            .tabs()
            .iter()
            .filter(|tab| tab.process.is_some())
            .count();
        let download_count = downloads.items().len();
        let completed_download_count = downloads
            .items()
            .iter()
            .filter(|item| item.state == crate::download_manager::DownloadState::Completed)
            .count();

        Some(TaskManagerSnapshot {
            tab_count,
            process_count,
            download_count,
            completed_download_count,
        })
    }

    pub fn render(snapshot: &TaskManagerSnapshot) -> String {
        format!(
            "Task Manager\n- tabs: {}\n- active tab processes: {}\n- downloads: {} (completed: {})",
            snapshot.tab_count,
            snapshot.process_count,
            snapshot.download_count,
            snapshot.completed_download_count
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::download_manager::DownloadManagerService;
    use crate::settings_system::BrowserSettings;
    use crate::tab_manager::TabManagerService;
    use crate::tab_process_registry::TabProcessRegistry;

    use super::TaskManagerService;

    #[test]
    fn task_manager_observes_tabs_and_downloads() {
        let settings = BrowserSettings::default();
        let mut tabs = TabManagerService::default();
        let tab = tabs.open_new_tab("monitor");
        let mut registry = TabProcessRegistry::default();
        tabs.attach_process(tab.id, registry.register_thread_process_for_tab(tab.id));

        let mut downloads = DownloadManagerService::default();
        downloads.enqueue("demo.bin", "https://example.com/demo.bin");
        downloads.start_all(&settings);

        let snapshot = TaskManagerService
            .snapshot(&tabs, &downloads, &settings)
            .expect("task manager enabled");
        assert_eq!(snapshot.tab_count, 1);
        assert_eq!(snapshot.process_count, 1);
        assert_eq!(snapshot.completed_download_count, 1);
    }
}
