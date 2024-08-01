pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(b: u8, g: u8, r: u8) -> Self {
        Self { b, g, r }
    }

    pub fn to_hex(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }

    pub fn white() -> Self {
        Self::new(255, 255, 255)
    }

    pub fn darkblue() -> Self {
        Self::new(110, 52, 3)
    }


}