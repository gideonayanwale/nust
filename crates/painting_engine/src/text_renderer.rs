use crate::paint_commands::PaintCommand;
use layout_engine::layout_tree::LayoutTree;

pub fn render_text(tree: &LayoutTree) -> Vec<PaintCommand> {
    tree.blocks
        .iter()
        .map(|block| PaintCommand::DrawText {
            x: block.x,
            y: block.y,
            text: block.text.clone(),
        })
        .collect()
}
