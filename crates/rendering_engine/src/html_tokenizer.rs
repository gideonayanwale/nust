pub use html_tokenizer::tokenizer::HtmlToken;

pub fn tokenize(input: &str) -> Vec<HtmlToken> {
    html_tokenizer::tokenizer::tokenize(input)
}
