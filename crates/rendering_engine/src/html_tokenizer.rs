#[derive(Debug, Clone)]
pub enum HtmlToken {
    Text(String),
    Tag(String),
}

pub fn tokenize(input: &str) -> Vec<HtmlToken> {
    let mut tokens = Vec::new();
    let mut buf = String::new();
    let mut in_tag = false;

    for ch in input.chars() {
        match ch {
            '<' => {
                if !buf.trim().is_empty() {
                    tokens.push(HtmlToken::Text(buf.trim().to_string()));
                }
                buf.clear();
                in_tag = true;
            }
            '>' => {
                if in_tag && !buf.trim().is_empty() {
                    tokens.push(HtmlToken::Tag(buf.trim().to_string()));
                }
                buf.clear();
                in_tag = false;
            }
            _ => buf.push(ch),
        }
    }

    if !buf.trim().is_empty() && !in_tag {
        tokens.push(HtmlToken::Text(buf.trim().to_string()));
    }

    tokens
}
