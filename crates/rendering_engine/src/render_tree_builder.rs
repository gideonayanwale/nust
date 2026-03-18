use dom_engine::document::Document;

pub use render_tree::model::{RenderNode, RenderTree};

pub fn build_from_document(document: &Document) -> RenderTree {
    render_tree::builder::build_render_tree(document)
}
