#[derive(Copy, Clone)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    pub fn from_rgb(red: u8, green: u8, blue: u8) -> Color {
        Color { red, green, blue }
    }
}

impl Into<(f32, f32, f32, f32)> for Color {
    fn into(self) -> (f32, f32, f32, f32) {
        (
            f32::from(self.red) / 256.0,
            f32::from(self.green) / 256.0,
            f32::from(self.blue) / 256.0,
            1.0,
        )
    }
}
