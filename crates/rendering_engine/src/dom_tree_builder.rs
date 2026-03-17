use dom_engine::document::Document;

pub fn build_document(text_nodes: &[String]) -> Document {
    let mut doc = Document::new();
    for text in text_nodes {
        doc.append_text(text.clone());
    }
    doc
}
