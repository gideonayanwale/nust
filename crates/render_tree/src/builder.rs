use dom_engine::document::Document;

use crate::model::{RenderNode, RenderTree};

pub fn build_render_tree(document: &Document) -> RenderTree {
    let mut tree = RenderTree::default();
    for line in document.text_content().lines().filter(|l| !l.is_empty()) {
        tree.nodes.push(RenderNode {
            text: line.to_string(),
        });
    }
    tree
}
