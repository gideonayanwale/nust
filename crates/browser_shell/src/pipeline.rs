use html_parser::parser::parse_to_document;
use html_tokenizer::tokenizer::tokenize;
use layout_engine::block_layout::compute_block_layout;
use paint_engine::commands::PaintCommand;
use paint_engine::painter::generate_paint_commands;
use render_tree::builder::build_render_tree;
use resource_loader::loader::ResourceLoader;

#[derive(Debug)]
pub struct PipelineOutput {
    pub commands: Vec<PaintCommand>,
}

pub fn run_pipeline(url: &str) -> Result<PipelineOutput, String> {
    let loader = ResourceLoader::default();
    let response = loader.load(url)?;
    let tokens = tokenize(&response.body);
    let document = parse_to_document(&tokens);
    let render_tree = build_render_tree(&document);

    // Phase-1 layout continues to operate over document text content.
    // Render tree generation is included as a dedicated pipeline stage.
    let _node_count = render_tree.nodes.len();
    let layout = compute_block_layout(&document, 1024.0);
    let commands = generate_paint_commands(&layout);

    Ok(PipelineOutput { commands })
}
