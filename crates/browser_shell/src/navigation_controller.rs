//! Navigation controller for shell routes.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BrowserRoute {
    Home,
    NewTab,
    External(String),
}

#[derive(Debug, Default)]
pub struct NavigationControllerService;

impl NavigationControllerService {
    pub fn resolve_target(&self, input: &str) -> BrowserRoute {
        match input.trim() {
            "nust://home" | "home" => BrowserRoute::Home,
            "nust://new-tab" | "new-tab" => BrowserRoute::NewTab,
            other => BrowserRoute::External(other.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{BrowserRoute, NavigationControllerService};

    #[test]
    fn resolves_home_and_new_tab_routes() {
        let controller = NavigationControllerService;
        assert_eq!(controller.resolve_target("home"), BrowserRoute::Home);
        assert_eq!(controller.resolve_target("new-tab"), BrowserRoute::NewTab);
        assert_eq!(
            controller.resolve_target("https://example.com"),
            BrowserRoute::External("https://example.com".to_string())
        );
    }
}
<<<<<<< HEAD
=======
//! Navigation Controller module scaffold.

#[derive(Debug, Default)]
pub struct NavigationControllerService;
>>>>>>> main
