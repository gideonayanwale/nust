#[derive(Debug, Clone)]
pub struct LayoutBlock {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub text: String,
}

#[derive(Debug, Clone, Default)]
pub struct LayoutTree {
    pub blocks: Vec<LayoutBlock>,
}
