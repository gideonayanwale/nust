//! Fancy UI skin primitives for browser shell HTML surfaces.

use crate::design_system::DesignSystem;

#[derive(Debug, Clone)]
pub struct UiSkin {
    pub brand: &'static str,
    pub design: DesignSystem,
}

impl Default for UiSkin {
    fn default() -> Self {
        Self {
            brand: "NUST Aurora",
            design: DesignSystem::default(),
        }
    }
}

impl UiSkin {
    pub fn base_css(&self) -> String {
        format!(
            "{}\nbody{{margin:0;font-family:var(--n-font);background:linear-gradient(140deg,var(--n-bg-start) 0%,var(--n-bg-end) 100%);color:var(--n-text);}}\n.shell-root{{min-height:100vh;display:grid;place-items:center;padding:var(--n-space-lg);}}\n.card{{background:var(--n-panel);border:1px solid rgba(255,255,255,.35);border-radius:var(--n-radius-lg);box-shadow:0 16px 40px rgba(2,6,23,.25);backdrop-filter:blur(8px);}}\n.button{{background:var(--n-primary);color:white;border:0;border-radius:var(--n-radius-sm);padding:10px 16px;font-weight:600;cursor:pointer;}}\n.button:hover{{filter:brightness(1.07);}}\n.chip{{display:inline-block;padding:6px 10px;border-radius:var(--n-radius-pill);background:var(--n-primary-soft);color:var(--n-primary);font-weight:600;}}\n.surface-title{{font-size:var(--n-type-title);margin:0;}}\n.surface-subtitle{{font-size:var(--n-type-caption);color:var(--n-text-muted);margin:0;}}",
            self.design.css_variables()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::UiSkin;

    #[test]
    fn exposes_base_css_tokens() {
        let css = UiSkin::default().base_css();
        assert!(css.contains("linear-gradient"));
        assert!(css.contains(".card"));
        assert!(css.contains("--n-primary"));
    }
}
