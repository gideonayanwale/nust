#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HtmlToken {
    Text(String),
    StartTag(String),
    EndTag(String),
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
                if in_tag {
                    let tag = buf.trim();
                    if let Some(stripped) = tag.strip_prefix('/') {
                        tokens.push(HtmlToken::EndTag(stripped.trim().to_string()));
                    } else if !tag.is_empty() {
                        tokens.push(HtmlToken::StartTag(tag.to_string()));
                    }
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

#[cfg(test)]
mod tests {
    use super::{tokenize, HtmlToken};

    #[test]
    fn tokenizes_basic_html() {
        let t = tokenize("<h1>Hello</h1>");
        assert!(t.contains(&HtmlToken::StartTag("h1".to_string())));
        assert!(t.contains(&HtmlToken::Text("Hello".to_string())));
        assert!(t.contains(&HtmlToken::EndTag("h1".to_string())));
    }
}
