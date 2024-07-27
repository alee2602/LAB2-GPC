use minifb::{Key, Window, WindowOptions};
use std::time::Duration;

mod color;
mod framebuffer;
use crate::color::Color;
use crate::framebuffer::Framebuffer;

fn main() {
    let window_width = 800;
    let window_height = 600;
    let framebuffer_width = 100;
    let framebuffer_height = 100;

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Conway's Game of Life - Lab 2",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    framebuffer.set_background_color(Color::white());
    framebuffer.set_current_color(Color::red());

    // Dibuja un punto en el centro
    framebuffer.point(framebuffer_width / 2, framebuffer_height / 2);

    let frame_delay = Duration::from_millis(100);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Actualizar el framebuffer
        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
