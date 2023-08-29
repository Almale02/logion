#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, Default)]
pub struct USVec2 {
    pub x: usize,
    pub y: usize,
}
impl USVec2 {
    pub fn new(x: usize, y: usize) -> Self {
        USVec2 { x, y }
    }
}
