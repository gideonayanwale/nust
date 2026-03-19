//! Centralized design system tokens for browser shell surfaces.

#[derive(Debug, Clone)]
pub struct ColorPalette {
    pub bg_start: &'static str,
    pub bg_end: &'static str,
    pub panel: &'static str,
    pub text: &'static str,
    pub text_muted: &'static str,
    pub primary: &'static str,
    pub primary_soft: &'static str,
    pub border: &'static str,
}

#[derive(Debug, Clone)]
pub struct SpacingScale {
    pub xs: u8,
    pub sm: u8,
    pub md: u8,
    pub lg: u8,
    pub xl: u8,
}

#[derive(Debug, Clone)]
pub struct RadiusScale {
    pub sm: u8,
    pub md: u8,
    pub lg: u8,
    pub pill: u16,
}

#[derive(Debug, Clone)]
pub struct TypographyScale {
    pub family: &'static str,
    pub hero: u8,
    pub title: u8,
    pub body: u8,
    pub caption: u8,
}

#[derive(Debug, Clone)]
pub struct DesignSystem {
    pub colors: ColorPalette,
    pub spacing: SpacingScale,
    pub radius: RadiusScale,
    pub type_scale: TypographyScale,
}

impl Default for DesignSystem {
    fn default() -> Self {
        Self {
            colors: ColorPalette {
                bg_start: "#0f172a",
                bg_end: "#1d4ed8",
                panel: "rgba(255,255,255,0.92)",
                text: "#0b1220",
                text_muted: "#4b5563",
                primary: "#2563eb",
                primary_soft: "rgba(37,99,235,0.12)",
                border: "#d0d7de",
            },
            spacing: SpacingScale {
                xs: 6,
                sm: 10,
                md: 16,
                lg: 24,
                xl: 32,
            },
            radius: RadiusScale {
                sm: 10,
                md: 14,
                lg: 20,
                pill: 999,
            },
            type_scale: TypographyScale {
                family: "Inter,Segoe UI,Arial,sans-serif",
                hero: 34,
                title: 28,
                body: 16,
                caption: 13,
            },
        }
    }
}

impl DesignSystem {
    pub fn css_variables(&self) -> String {
        format!(
            ":root{{--n-bg-start:{};--n-bg-end:{};--n-panel:{};--n-text:{};--n-text-muted:{};--n-primary:{};--n-primary-soft:{};--n-border:{};--n-space-xs:{}px;--n-space-sm:{}px;--n-space-md:{}px;--n-space-lg:{}px;--n-space-xl:{}px;--n-radius-sm:{}px;--n-radius-md:{}px;--n-radius-lg:{}px;--n-radius-pill:{};--n-font:{};--n-type-hero:{}px;--n-type-title:{}px;--n-type-body:{}px;--n-type-caption:{}px;}}",
            self.colors.bg_start,
            self.colors.bg_end,
            self.colors.panel,
            self.colors.text,
            self.colors.text_muted,
            self.colors.primary,
            self.colors.primary_soft,
            self.colors.border,
            self.spacing.xs,
            self.spacing.sm,
            self.spacing.md,
            self.spacing.lg,
            self.spacing.xl,
            self.radius.sm,
            self.radius.md,
            self.radius.lg,
            self.radius.pill,
            self.type_scale.family,
            self.type_scale.hero,
            self.type_scale.title,
            self.type_scale.body,
            self.type_scale.caption,
        )
    }
}
