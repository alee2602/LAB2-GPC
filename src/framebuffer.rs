use crate::color::Color;
pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
    background_color: Color,
    current_color: Color,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![0; width * height],
            background_color: Color::black(),
            current_color: Color::white(),
        }
    }

    pub fn clear(&mut self) {
        for pixel in self.buffer.iter_mut() {
            *pixel = self.background_color.to_hex();
        }
    }

    pub fn point(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            self.buffer[index] = self.current_color.to_hex();
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }
}

pub fn count_live_neighbors(framebuffer: &Framebuffer, x: usize, y: usize) -> usize {
    let directions = [
        (-1, -1), (0, -1), (1, -1),
        (-1, 0),         (1, 0),
        (-1, 1), (0, 1), (1, 1),
    ];

    directions.iter().filter(|&&(dx, dy)| {
        let nx = x.wrapping_add(dx as usize);
        let ny = y.wrapping_add(dy as usize);
        nx < framebuffer.width && ny < framebuffer.height && framebuffer.buffer[ny * framebuffer.width + nx] == 0xFFFFFF
    }).count()
}

pub fn update_game(framebuffer: &mut Framebuffer) {
    let mut new_buffer = framebuffer.buffer.clone();

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let live_neighbors = count_live_neighbors(framebuffer, x, y);
            let idx = y * framebuffer.width + x;
            let is_alive = framebuffer.buffer[idx] == 0xFFFFFF;

            new_buffer[idx] = match (is_alive, live_neighbors) {
                (true, 2) | (true, 3) => 0xFFFFFF, // Sobrevive
                (false, 3) => 0xFFFFFF,            // Se reproduce
                _ => 0x000000,                     // Muere
            };
        }
    }

    framebuffer.buffer = new_buffer;
}
