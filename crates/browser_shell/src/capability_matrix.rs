//! Chrome-parity inspired capability matrix with lightweight posture.

use crate::settings_system::BrowserSettings;

#[derive(Debug, Clone)]
pub struct CapabilityStatus {
    pub feature: &'static str,
    pub available: bool,
    pub notes: &'static str,
}

#[derive(Debug, Clone)]
pub struct CapabilityMatrix {
    pub capabilities: Vec<CapabilityStatus>,
}

impl CapabilityMatrix {
    pub fn baseline(settings: &BrowserSettings) -> Self {
        Self {
            capabilities: vec![
                CapabilityStatus {
                    feature: "Tabs + history + bookmarks",
                    available: true,
                    notes: "Implemented in browser_shell state managers",
                },
                CapabilityStatus {
                    feature: "HTML pipeline",
                    available: true,
                    notes: "Tokenization -> DOM -> layout -> paint commands",
                },
                CapabilityStatus {
                    feature: "Incognito mode",
                    available: true,
                    notes: "Session mode toggle implemented",
                },
                CapabilityStatus {
                    feature: "Predictive preload",
                    available: settings.preload_pages_enabled,
                    notes: "Disabled by default for lightweight mode",
                },
                CapabilityStatus {
                    feature: "Extension runtime",
                    available: settings.extensions_enabled,
                    notes: "Scaffold present; runtime behavior partial",
                },
                CapabilityStatus {
                    feature: "Per-tab thread process registry",
                    available: settings.per_tab_thread_process_enabled,
                    notes: "Each tab can be attached to a dedicated worker thread record",
                },
                CapabilityStatus {
                    feature: "Automation workflows",
                    available: settings.automation_registry_enabled,
                    notes: "Default automation scripts can be registered per tab",
                },
                CapabilityStatus {
                    feature: "Premium download tray",
                    available: settings.downloads_enabled,
                    notes: "Queue with acceleration profile controlled by settings",
                },
                CapabilityStatus {
                    feature: "Extension library (app-wide)",
                    available: settings.extension_library_enabled
                        && settings.global_extension_runtime_enabled,
                    notes: "Installed extensions can activate across the shell",
                },
                CapabilityStatus {
                    feature: "Task manager",
                    available: settings.task_manager_enabled,
                    notes: "Monitors tabs, tab processes, and downloads",
                },
                CapabilityStatus {
                    feature: "Upload-on-demand tray",
                    available: settings.upload_on_demand_enabled,
                    notes: "Supports arbitrary file types with configurable upload latency",
                },
                CapabilityStatus {
                    feature: "GPU compositing",
                    available: true,
                    notes: "Compositor crate scaffold present",
                },
                CapabilityStatus {
                    feature: "JavaScript runtime",
                    available: true,
                    notes: "Runtime crate scaffold present",
                },
            ],
        }
    }

    pub fn to_report(&self) -> String {
        let mut out =
            String::from("NUST Capability Report (lightweight Chrome-feature trajectory)\n");
        for capability in &self.capabilities {
            let status = if capability.available { "✅" } else { "⚪" };
            out.push_str(&format!(
                "- {status} {} — {}\n",
                capability.feature, capability.notes
            ));
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use crate::settings_system::BrowserSettings;

    use super::CapabilityMatrix;

    #[test]
    fn report_contains_key_features() {
        let settings = BrowserSettings::default();
        let report = CapabilityMatrix::baseline(&settings).to_report();
        assert!(report.contains("Tabs + history + bookmarks"));
        assert!(report.contains("HTML pipeline"));
    }
}
