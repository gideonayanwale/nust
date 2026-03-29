//! Download tray and accelerated transfer orchestration.

use crate::settings_system::BrowserSettings;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DownloadState {
    Queued,
    Downloading,
    Completed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpeedProfile {
    Efficient,
    PremiumTurbo,
}

#[derive(Debug, Clone)]
pub struct DownloadItem {
    pub id: usize,
    pub file_name: String,
    pub source_url: String,
    pub speed_mbps: u32,
    pub state: DownloadState,
}

#[derive(Debug, Default)]
pub struct DownloadManagerService {
    queue: Vec<DownloadItem>,
    next_id: usize,
}

impl DownloadManagerService {
    pub fn enqueue(
        &mut self,
        file_name: impl Into<String>,
        source_url: impl Into<String>,
    ) -> DownloadItem {
        let item = DownloadItem {
            id: self.next_id,
            file_name: file_name.into(),
            source_url: source_url.into(),
            speed_mbps: 0,
            state: DownloadState::Queued,
        };
        self.next_id += 1;
        self.queue.push(item.clone());
        item
    }

    pub fn start_all(&mut self, settings: &BrowserSettings) {
        if !settings.downloads_enabled {
            return;
        }

        let speed_profile = if settings.premium_download_acceleration_enabled {
            SpeedProfile::PremiumTurbo
        } else {
            SpeedProfile::Efficient
        };

        for item in &mut self.queue {
            item.state = DownloadState::Downloading;
            item.speed_mbps = match speed_profile {
                SpeedProfile::Efficient => 140,
                SpeedProfile::PremiumTurbo => 420,
            };
            item.state = DownloadState::Completed;
        }
    }

    pub fn tray_summary(&self) -> String {
        let total = self.queue.len();
        let completed = self
            .queue
            .iter()
            .filter(|item| item.state == DownloadState::Completed)
            .count();
        let peak_speed = self
            .queue
            .iter()
            .map(|item| item.speed_mbps)
            .max()
            .unwrap_or(0);

        format!("Download Tray: {completed}/{total} completed · peak speed {peak_speed} Mbps")
    }

    pub fn items(&self) -> &[DownloadItem] {
        &self.queue
    }
}

#[cfg(test)]
mod tests {
    use crate::settings_system::{BrowserSettings, PerformanceMode};

    use super::DownloadManagerService;

    #[test]
    fn premium_profile_is_faster_than_efficient() {
        let mut fast_settings = BrowserSettings::default();
        fast_settings.apply_mode(PerformanceMode::Balanced);

        let mut slow_settings = BrowserSettings::default();
        slow_settings.apply_mode(PerformanceMode::Lightweight);

        let mut fast_manager = DownloadManagerService::default();
        fast_manager.enqueue("build.zip", "https://cdn.example.com/build.zip");
        fast_manager.start_all(&fast_settings);

        let mut slow_manager = DownloadManagerService::default();
        slow_manager.enqueue("build.zip", "https://cdn.example.com/build.zip");
        slow_manager.start_all(&slow_settings);

        assert!(fast_manager.items()[0].speed_mbps > slow_manager.items()[0].speed_mbps);
    }
}
