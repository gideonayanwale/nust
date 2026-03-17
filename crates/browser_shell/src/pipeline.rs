use layout_engine::block_layout::compute_block_layout;
use networking_stack::http1_client::{HttpClient, NativeHttp1Client};
use painting_engine::paint_commands::PaintCommand;
use painting_engine::text_renderer::render_text;
use rendering_engine::dom_tree_builder::build_document;
use rendering_engine::html_parser::extract_text_nodes;
use rendering_engine::html_tokenizer::tokenize;

#[derive(Debug)]
pub struct PipelineOutput {
    pub commands: Vec<PaintCommand>,
}

pub fn run_pipeline(url: &str) -> Result<PipelineOutput, String> {
    let client = NativeHttp1Client;
    let response = client.fetch(url)?;
    let tokens = tokenize(&response.body);
    let text_nodes = extract_text_nodes(&tokens);
    let document = build_document(&text_nodes);
    let layout = compute_block_layout(&document, 1024.0);
    let commands = render_text(&layout);

    Ok(PipelineOutput { commands })
}
