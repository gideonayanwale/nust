use dom_engine::document::Document;
use html_tokenizer::tokenizer::HtmlToken;

pub fn parse_to_document(tokens: &[HtmlToken]) -> Document {
    let mut document = Document::new();

    for token in tokens {
        if let HtmlToken::Text(text) = token {
            if !text.trim().is_empty() {
                document.append_text(text.clone());
            }
        }
    }

    document
}

#[cfg(test)]
mod tests {
    use super::parse_to_document;
    use html_tokenizer::tokenizer::{tokenize, HtmlToken};

    #[test]
    fn parses_text_nodes_into_document() {
        let tokens = vec![
            HtmlToken::Text("Hello".to_string()),
            HtmlToken::Text("World".to_string()),
        ];
        let doc = parse_to_document(&tokens);
        assert!(doc.text_content().contains("Hello"));
    }

    #[test]
    fn integrates_with_tokenizer() {
        let tokens = tokenize("<p>NUST</p>");
        let doc = parse_to_document(&tokens);
        assert_eq!(doc.text_content(), "NUST");
    }
}
