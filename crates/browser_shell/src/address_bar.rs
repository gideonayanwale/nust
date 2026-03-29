//! Address/search bar parser.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddressInput {
    Url(String),
    SearchQuery(String),
}

#[derive(Debug, Default)]
pub struct AddressBarService;

impl AddressBarService {
    pub fn parse(&self, input: &str) -> AddressInput {
        let trimmed = input.trim();
        if trimmed.starts_with("http://")
            || trimmed.starts_with("https://")
            || trimmed.starts_with("nust://")
        {
            AddressInput::Url(trimmed.to_string())
        } else {
            AddressInput::SearchQuery(trimmed.to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{AddressBarService, AddressInput};

    #[test]
    fn parses_url_or_search_input() {
        let service = AddressBarService;
        assert_eq!(
            service.parse("https://nust.dev"),
            AddressInput::Url("https://nust.dev".to_string())
        );
        assert_eq!(
            service.parse("best rust browser architecture"),
            AddressInput::SearchQuery("best rust browser architecture".to_string())
        );
    }
}
