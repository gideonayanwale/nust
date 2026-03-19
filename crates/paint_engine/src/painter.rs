use layout_engine::layout_tree::LayoutTree;

use crate::commands::PaintCommand;

pub fn generate_paint_commands(layout: &LayoutTree) -> Vec<PaintCommand> {
    layout
        .blocks
        .iter()
        .map(|block| PaintCommand::DrawText {
            x: block.x,
            y: block.y,
            text: block.text.clone(),
        })
        .collect()
}
