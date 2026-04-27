

#[derive(Clone, Copy, Debug)]
pub struct BorderStyle {
    pub color: [u8; 4],
    pub width: f32,
    pub radius: f32,
}

impl Default for BorderStyle {
    fn default() -> Self {
        Self {
            color: [0, 0, 0, 0],
            width: 0.0,
            radius: 0.0,
        }
    }
}