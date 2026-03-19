use layout_engine::block_layout::compute_block_layout;
use networking_stack::http1_client::{HttpClient, NativeHttp1Client};
use painting_engine::paint_commands::PaintCommand;
use painting_engine::text_renderer::render_text;
<<<<<<< HEAD
use rendering_engine::html_parser::parse_to_document;
use rendering_engine::html_tokenizer::tokenize;
use rendering_engine::render_tree_builder::build_from_document;
=======
use rendering_engine::dom_tree_builder::build_document;
use rendering_engine::html_parser::extract_text_nodes;
use rendering_engine::html_tokenizer::tokenize;
>>>>>>> main

#[derive(Debug)]
pub struct PipelineOutput {
    pub commands: Vec<PaintCommand>,
}

pub fn run_pipeline(url: &str) -> Result<PipelineOutput, String> {
    let client = NativeHttp1Client;
    let response = client.fetch(url)?;
    let tokens = tokenize(&response.body);
<<<<<<< HEAD
    let document = parse_to_document(&tokens);
    let render_tree = build_from_document(&document);

    // Phase-1 layout continues to operate over document text content.
    let _node_count = render_tree.nodes.len();
=======
    let text_nodes = extract_text_nodes(&tokens);
    let document = build_document(&text_nodes);
>>>>>>> main
    let layout = compute_block_layout(&document, 1024.0);
    let commands = render_text(&layout);

    Ok(PipelineOutput { commands })
}
