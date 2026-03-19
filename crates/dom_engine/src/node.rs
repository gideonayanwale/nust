#[derive(Debug, Clone)]
pub enum NodeKind {
    Document,
    Element { tag: String },
    Text { content: String },
}

#[derive(Debug, Clone)]
pub struct Node {
    pub kind: NodeKind,
    pub children: Vec<Node>,
}

impl Node {
    pub fn new(kind: NodeKind) -> Self {
        Self {
            kind,
            children: Vec::new(),
        }
    }

    pub fn push_child(&mut self, child: Node) {
        self.children.push(child);
    }
}
