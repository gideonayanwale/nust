#[derive(Debug, Clone)]
pub enum PaintCommand {
    DrawText { x: f32, y: f32, text: String },
}
