//! Lightweight settings system for feature tuning.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PerformanceMode {
    Balanced,
    Lightweight,
    MaximumCompatibility,
}

#[derive(Debug, Clone)]
pub struct BrowserSettings {
    pub performance_mode: PerformanceMode,
    pub tab_discarding_enabled: bool,
    pub preload_pages_enabled: bool,
    pub safe_browsing_enabled: bool,
    pub extensions_enabled: bool,
}

impl Default for BrowserSettings {
    fn default() -> Self {
        Self {
            performance_mode: PerformanceMode::Lightweight,
            tab_discarding_enabled: true,
            preload_pages_enabled: false,
            safe_browsing_enabled: true,
            extensions_enabled: true,
        }
    }
}

impl BrowserSettings {
    pub fn apply_mode(&mut self, mode: PerformanceMode) {
        self.performance_mode = mode.clone();
        match mode {
            PerformanceMode::Lightweight => {
                self.tab_discarding_enabled = true;
                self.preload_pages_enabled = false;
            }
            PerformanceMode::Balanced => {
                self.tab_discarding_enabled = true;
                self.preload_pages_enabled = true;
            }
            PerformanceMode::MaximumCompatibility => {
                self.tab_discarding_enabled = false;
                self.preload_pages_enabled = true;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{BrowserSettings, PerformanceMode};

    #[test]
    fn lightweight_mode_prefers_lower_resource_usage() {
        let mut settings = BrowserSettings::default();
        settings.apply_mode(PerformanceMode::Lightweight);
        assert!(settings.tab_discarding_enabled);
        assert!(!settings.preload_pages_enabled);
    }
}
