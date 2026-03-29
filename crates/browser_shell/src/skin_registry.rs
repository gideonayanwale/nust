//! Registry of shell skins.

use crate::settings_system::SkinMode;
use crate::ui_skin::UiSkin;

pub struct SkinRegistry;

impl SkinRegistry {
    pub fn resolve(mode: &SkinMode) -> UiSkin {
        match mode {
            SkinMode::NustAurora => UiSkin::aurora(),
            SkinMode::EdgeChromeBraveFusion => UiSkin::edge_chrome_brave_fusion(),
        }
    }
}
