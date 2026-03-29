//! Extension library and global runtime wiring.

use std::collections::HashSet;

use crate::settings_system::BrowserSettings;

#[derive(Debug, Clone)]
pub struct ExtensionPackage {
    pub id: String,
    pub display_name: String,
    pub enabled: bool,
}

#[derive(Debug, Default)]
pub struct ExtensionLibraryService {
    packages: Vec<ExtensionPackage>,
    active_scope: HashSet<String>,
}

impl ExtensionLibraryService {
    pub fn install(&mut self, id: impl Into<String>, display_name: impl Into<String>) {
        self.packages.push(ExtensionPackage {
            id: id.into(),
            display_name: display_name.into(),
            enabled: true,
        });
    }

    pub fn activate_globally(&mut self, settings: &BrowserSettings) {
        if !settings.extension_library_enabled || !settings.global_extension_runtime_enabled {
            return;
        }
        self.active_scope = self.packages.iter().map(|pkg| pkg.id.clone()).collect();
    }

    pub fn summary(&self) -> String {
        format!(
            "Extension Library: {} installed · {} active app-wide",
            self.packages.len(),
            self.active_scope.len()
        )
    }

    pub fn installed(&self) -> &[ExtensionPackage] {
        &self.packages
    }
}

#[cfg(test)]
mod tests {
    use crate::settings_system::BrowserSettings;

    use super::ExtensionLibraryService;

    #[test]
    fn extensions_can_activate_globally() {
        let settings = BrowserSettings::default();
        let mut service = ExtensionLibraryService::default();
        service.install("uBlock-lite", "uBlock Lite");
        service.install("vim-nav", "Vim Navigator");
        service.activate_globally(&settings);
        assert!(service.summary().contains("2 active app-wide"));
    }
}
