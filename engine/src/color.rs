#[derive(Copy, Clone)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    pub fn from_rgb(red: u8, green: u8, blue: u8) -> Color {
        Color {
            red,
            green,
            blue,
        }
    }
}

impl Into<(f32, f32, f32, f32)> for Color {
    fn into(self) -> (f32, f32, f32, f32) {
        (self.red as f32 / 256.0, self.green as f32 / 256.0, self.blue as f32 / 256.0, 1.0)
    }
}
