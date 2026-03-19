#[derive(Debug, Clone)]
pub struct RenderNode {
    pub text: String,
}

#[derive(Debug, Clone, Default)]
pub struct RenderTree {
    pub nodes: Vec<RenderNode>,
}
