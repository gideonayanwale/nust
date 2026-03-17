use dom_engine::document::Document;

use crate::layout_tree::{LayoutBlock, LayoutTree};

pub fn compute_block_layout(document: &Document, viewport_width: f32) -> LayoutTree {
    let mut y = 0.0;
    let line_height = 20.0;
    let mut tree = LayoutTree::default();

    for line in document
        .text_content()
        .lines()
        .filter(|line| !line.is_empty())
    {
        tree.blocks.push(LayoutBlock {
            x: 16.0,
            y,
            width: viewport_width - 32.0,
            height: line_height,
            text: line.to_string(),
        });
        y += line_height;
    }

    tree
}
