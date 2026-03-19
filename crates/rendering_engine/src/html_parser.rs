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
<<<<<<< HEAD

pub fn parse_to_document(tokens: &[HtmlToken]) -> dom_engine::document::Document {
    html_parser::parser::parse_to_document(tokens)
}
=======
>>>>>>> main
