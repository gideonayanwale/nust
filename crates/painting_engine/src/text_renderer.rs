use crate::paint_commands::PaintCommand;
use layout_engine::layout_tree::LayoutTree;

pub fn render_text(tree: &LayoutTree) -> Vec<PaintCommand> {
    paint_engine::painter::generate_paint_commands(tree)
}
