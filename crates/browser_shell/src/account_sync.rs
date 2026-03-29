//! Sync configured state to user account.

use crate::settings_system::BrowserSettings;
use crate::tab_manager::TabManagerService;

#[derive(Debug, Clone)]
pub struct SyncedState {
    pub account_id: String,
    pub open_tab_count: usize,
    pub split_view_count: usize,
    pub uploads_enabled: bool,
    pub downloads_enabled: bool,
    pub extension_library_enabled: bool,
}

#[derive(Debug, Default)]
pub struct AccountSyncService {
    pub last_synced: Option<SyncedState>,
}

impl AccountSyncService {
    pub fn sync_to_account(
        &mut self,
        account_id: impl Into<String>,
        settings: &BrowserSettings,
        tabs: &TabManagerService,
    ) -> Option<SyncedState> {
        if !settings.account_sync_enabled {
            return None;
        }
        let snapshot = SyncedState {
            account_id: account_id.into(),
            open_tab_count: tabs.tabs().len(),
            split_view_count: tabs.split_groups().len(),
            uploads_enabled: settings.upload_on_demand_enabled,
            downloads_enabled: settings.downloads_enabled,
            extension_library_enabled: settings.extension_library_enabled,
        };
        self.last_synced = Some(snapshot.clone());
        Some(snapshot)
    }
}

#[cfg(test)]
mod tests {
    use crate::settings_system::BrowserSettings;
    use crate::tab_manager::TabManagerService;

    use super::AccountSyncService;

    #[test]
    fn syncs_core_state_to_account() {
        let settings = BrowserSettings::default();
        let mut tabs = TabManagerService::default();
        tabs.open_new_tab("sync");
        let mut sync = AccountSyncService::default();
        let snapshot = sync
            .sync_to_account("user-42", &settings, &tabs)
            .expect("sync enabled");
        assert_eq!(snapshot.account_id, "user-42");
        assert_eq!(snapshot.open_tab_count, 1);
    }
}
