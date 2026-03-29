//! Lightweight settings system for feature tuning.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PerformanceMode {
    Balanced,
    Lightweight,
    MaximumCompatibility,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SkinMode {
    NustAurora,
    EdgeChromeBraveFusion,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BookmarkTabPosition {
    Top,
    Left,
    Right,
    Bottom,
}

#[derive(Debug, Clone)]
pub struct BrowserSettings {
    pub performance_mode: PerformanceMode,
    pub skin_mode: SkinMode,
    pub tab_discarding_enabled: bool,
    pub per_tab_thread_process_enabled: bool,
    pub preload_pages_enabled: bool,
    pub safe_browsing_enabled: bool,
    pub extensions_enabled: bool,
    pub automation_registry_enabled: bool,
    pub automation_autopilot_enabled: bool,
    pub downloads_enabled: bool,
    pub premium_download_acceleration_enabled: bool,
    pub extension_library_enabled: bool,
    pub global_extension_runtime_enabled: bool,
    pub task_manager_enabled: bool,
    pub upload_on_demand_enabled: bool,
    pub upload_latency_ms: u64,
    pub account_sync_enabled: bool,
    pub desktop_bookmark_tab_enabled: bool,
    pub desktop_bookmark_tab_position: BookmarkTabPosition,
}

impl Default for BrowserSettings {
    fn default() -> Self {
        Self {
            performance_mode: PerformanceMode::Lightweight,
            skin_mode: SkinMode::EdgeChromeBraveFusion,
            tab_discarding_enabled: true,
            per_tab_thread_process_enabled: true,
            preload_pages_enabled: false,
            safe_browsing_enabled: true,
            extensions_enabled: true,
            automation_registry_enabled: true,
            automation_autopilot_enabled: true,
            downloads_enabled: true,
            premium_download_acceleration_enabled: true,
            extension_library_enabled: true,
            global_extension_runtime_enabled: true,
            task_manager_enabled: true,
            upload_on_demand_enabled: true,
            upload_latency_ms: 180,
            account_sync_enabled: true,
            desktop_bookmark_tab_enabled: true,
            desktop_bookmark_tab_position: BookmarkTabPosition::Top,
        }
    }
}

impl BrowserSettings {
    pub fn apply_mode(&mut self, mode: PerformanceMode) {
        self.performance_mode = mode.clone();
        match mode {
            PerformanceMode::Lightweight => {
                self.tab_discarding_enabled = true;
                self.per_tab_thread_process_enabled = true;
                self.preload_pages_enabled = false;
                self.premium_download_acceleration_enabled = false;
                self.upload_latency_ms = 320;
            }
            PerformanceMode::Balanced => {
                self.tab_discarding_enabled = true;
                self.per_tab_thread_process_enabled = true;
                self.preload_pages_enabled = true;
                self.premium_download_acceleration_enabled = true;
                self.upload_latency_ms = 180;
            }
            PerformanceMode::MaximumCompatibility => {
                self.tab_discarding_enabled = false;
                self.per_tab_thread_process_enabled = true;
                self.preload_pages_enabled = true;
                self.premium_download_acceleration_enabled = true;
                self.upload_latency_ms = 120;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{BookmarkTabPosition, BrowserSettings, PerformanceMode, SkinMode};

    #[test]
    fn lightweight_mode_prefers_lower_resource_usage() {
        let mut settings = BrowserSettings::default();
        settings.apply_mode(PerformanceMode::Lightweight);
        assert!(settings.tab_discarding_enabled);
        assert!(!settings.preload_pages_enabled);
    }

    #[test]
    fn defaults_to_fusion_skin_and_automation() {
        let settings = BrowserSettings::default();
        assert_eq!(settings.skin_mode, SkinMode::EdgeChromeBraveFusion);
        assert!(settings.per_tab_thread_process_enabled);
        assert!(settings.automation_registry_enabled);
        assert!(settings.automation_autopilot_enabled);
        assert!(settings.downloads_enabled);
        assert!(settings.extension_library_enabled);
        assert!(settings.task_manager_enabled);
        assert!(settings.upload_on_demand_enabled);
        assert!(settings.upload_latency_ms > 0);
        assert!(settings.account_sync_enabled);
        assert!(settings.desktop_bookmark_tab_enabled);
        assert_eq!(
            settings.desktop_bookmark_tab_position,
            BookmarkTabPosition::Top
        );
    }
}
