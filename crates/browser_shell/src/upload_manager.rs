//! Upload-on-demand tray with configurable latency profile.

use crate::settings_system::BrowserSettings;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UploadState {
    Queued,
    Uploading,
    Completed,
}

#[derive(Debug, Clone)]
pub struct UploadItem {
    pub id: usize,
    pub file_name: String,
    pub mime_type: String,
    pub destination: String,
    pub latency_ms: u64,
    pub state: UploadState,
}

#[derive(Debug, Default)]
pub struct UploadManagerService {
    queue: Vec<UploadItem>,
    next_id: usize,
}

impl UploadManagerService {
    pub fn queue_on_demand(
        &mut self,
        file_name: impl Into<String>,
        mime_type: impl Into<String>,
        destination: impl Into<String>,
    ) -> UploadItem {
        let item = UploadItem {
            id: self.next_id,
            file_name: file_name.into(),
            mime_type: mime_type.into(),
            destination: destination.into(),
            latency_ms: 0,
            state: UploadState::Queued,
        };
        self.next_id += 1;
        self.queue.push(item.clone());
        item
    }

    pub fn run_on_demand(&mut self, settings: &BrowserSettings) {
        if !settings.upload_on_demand_enabled {
            return;
        }

        for item in &mut self.queue {
            item.state = UploadState::Uploading;
            item.latency_ms = settings.upload_latency_ms;
            item.state = UploadState::Completed;
        }
    }

    pub fn tray_summary(&self) -> String {
        let total = self.queue.len();
        let completed = self
            .queue
            .iter()
            .filter(|item| item.state == UploadState::Completed)
            .count();
        let max_latency = self
            .queue
            .iter()
            .map(|item| item.latency_ms)
            .max()
            .unwrap_or(0);

        format!(
            "Upload Tray (on-demand): {completed}/{total} completed · configured latency {max_latency} ms"
        )
    }

    pub fn items(&self) -> &[UploadItem] {
        &self.queue
    }
}

#[cfg(test)]
mod tests {
    use crate::settings_system::BrowserSettings;

    use super::{UploadManagerService, UploadState};

    #[test]
    fn upload_tray_supports_any_file_types() {
        let settings = BrowserSettings::default();
        let mut uploads = UploadManagerService::default();
        uploads.queue_on_demand(
            "movie.mkv",
            "video/x-matroska",
            "https://up.example.com/media",
        );
        uploads.queue_on_demand(
            "model.onnx",
            "application/octet-stream",
            "https://up.example.com/ml",
        );
        uploads.run_on_demand(&settings);

        assert_eq!(uploads.items().len(), 2);
        assert!(uploads
            .items()
            .iter()
            .all(|item| item.state == UploadState::Completed));
    }
}
