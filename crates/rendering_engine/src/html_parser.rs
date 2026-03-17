use crate::html_tokenizer::HtmlToken;

pub fn extract_text_nodes(tokens: &[HtmlToken]) -> Vec<String> {
    tokens
        .iter()
        .filter_map(|token| match token {
            HtmlToken::Text(text) if !text.trim().is_empty() => Some(text.trim().to_string()),
            _ => None,
        })
        .collect()
}
