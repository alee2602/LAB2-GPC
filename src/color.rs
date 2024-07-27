pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(b: u8, g: u8, r: u8) -> Self {
        Self { b, g, r }
    }

    pub fn from_hex(hex: u32) -> Self {
        Self {
            r: (hex & 0xFF) as u8,
            g: ((hex >> 8) & 0xFF) as u8,
            b: ((hex >> 16) & 0xFF) as u8,
            
        }
    }
    

    pub fn to_hex(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }

    pub fn white() -> Self {
        Self::new(255, 255, 255)
    }

    pub fn black() -> Self {
        Self::new(0, 0, 0)
    }

    pub fn yellow() -> Self {
        Self::new(255, 255, 0)
    }

    pub fn blue() -> Self {
        Self::new(0, 0, 255)
    }

    pub fn red() -> Self {
        Self::new(255, 0, 0)
    }

    pub fn green() -> Self {
        Self::new(0, 255, 0)
    }

}