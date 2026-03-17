use crate::node::{Node, NodeKind};

#[derive(Debug, Clone)]
pub struct Document {
    pub root: Node,
}

impl Document {
    pub fn new() -> Self {
        Self {
            root: Node::new(NodeKind::Document),
        }
    }

    pub fn append_text(&mut self, content: impl Into<String>) {
        self.root.push_child(Node::new(NodeKind::Text {
            content: content.into(),
        }));
    }

    pub fn text_content(&self) -> String {
        let mut out = String::new();
        for node in &self.root.children {
            if let NodeKind::Text { content } = &node.kind {
                if !out.is_empty() {
                    out.push('\n');
                }
                out.push_str(content.trim());
            }
        }
        out
    }
}
