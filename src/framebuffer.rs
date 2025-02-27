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
        let mut fb = Self {
            width,
            height,
            buffer: vec![0; width * height],
            background_color: Color::darkblue(),
            current_color: Color::white(),
        };
        fb.clear(); 
        fb
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
        self.clear();
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
        nx < framebuffer.width && ny < framebuffer.height && framebuffer.buffer[ny * framebuffer.width + nx] != framebuffer.background_color.to_hex()
    }).count()
}

pub fn update_game(framebuffer: &mut Framebuffer) {
    let mut new_buffer = framebuffer.buffer.clone();

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let live_neighbors = count_live_neighbors(framebuffer, x, y);
            let idx = y * framebuffer.width + x;
            let is_alive = framebuffer.buffer[idx] != framebuffer.background_color.to_hex();

            new_buffer[idx] = match (is_alive, live_neighbors) {
                (true, 2) => 0x4535C1, 
                (true, 3) => 0x478CCF, 
                (false, 3) => 0x77E4C8, 
                _ => framebuffer.background_color.to_hex(), 
            };
        }
    }

    framebuffer.buffer = new_buffer;
}





