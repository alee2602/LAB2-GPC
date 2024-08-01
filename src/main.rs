use minifb::{Key, Window, WindowOptions};
use std::time::Duration;

mod color;
mod framebuffer;
use crate::color::Color;
use crate::framebuffer::{Framebuffer, update_game};

fn initial_pattern(framebuffer: &mut Framebuffer) {
    
    pulsar(framebuffer, 5, 40);
    pulsar(framebuffer, 30, 40);
    pulsar(framebuffer, 55, 40);
    pulsar(framebuffer, 80, 40);

    schick_engine(framebuffer, 20, 25);
    schick_engine(framebuffer, 50, 25);
    schick_engine(framebuffer, 80, 25);

    gosper_glider_gun(framebuffer, 20, 45);
    gosper_glider_gun(framebuffer, 60, 45);

    glider(framebuffer, 40, 40);
    glider(framebuffer, 10, 15);
    glider(framebuffer, 80, 15);

    lightweight_spaceship(framebuffer, 50, 20);
    lightweight_spaceship(framebuffer, 50, 50);

    beehive(framebuffer, 10, 50);
    beehive(framebuffer, 30, 50);
    beehive(framebuffer, 50, 50);
    beehive(framebuffer, 70, 50);
    beehive(framebuffer, 90, 50);

    loaf(framebuffer, 30, 25);
    loaf(framebuffer, 55, 55);
    boat(framebuffer, 85, 55);

    penta_decathlon(framebuffer, 25, 50);
    penta_decathlon(framebuffer, 50, 50);
    penta_decathlon(framebuffer, 75, 50);


}


fn glider(framebuffer: &mut Framebuffer, x: usize, y: usize) {
    framebuffer.point(x, y);
    framebuffer.point(x + 1, y + 1);
    framebuffer.point(x + 2, y + 1);
    framebuffer.point(x, y + 2);
    framebuffer.point(x + 1, y + 2);
}

fn lightweight_spaceship(framebuffer: &mut Framebuffer, x: usize, y: usize) {
    framebuffer.point(x + 1, y);
    framebuffer.point(x + 4, y);
    framebuffer.point(x, y + 1);
    framebuffer.point(x, y + 2);
    framebuffer.point(x + 4, y + 2);
    framebuffer.point(x, y + 3);
    framebuffer.point(x + 1, y + 3);
    framebuffer.point(x + 2, y + 3);
    framebuffer.point(x + 3, y + 3);
}

fn beehive(framebuffer: &mut Framebuffer, x: usize, y: usize) {
    framebuffer.point(x + 1, y);
    framebuffer.point(x + 2, y);
    framebuffer.point(x, y + 1);
    framebuffer.point(x + 3, y + 1);
    framebuffer.point(x + 1, y + 2);
    framebuffer.point(x + 2, y + 2);
}

fn loaf(framebuffer: &mut Framebuffer, x: usize, y: usize) {
    framebuffer.point(x + 1, y);
    framebuffer.point(x + 2, y);
    framebuffer.point(x, y + 1);
    framebuffer.point(x + 3, y + 1);
    framebuffer.point(x + 1, y + 2);
    framebuffer.point(x + 3, y + 2);
    framebuffer.point(x + 2, y + 3);
}

fn boat(framebuffer: &mut Framebuffer, x: usize, y: usize) {
    framebuffer.point(x, y);
    framebuffer.point(x + 1, y);
    framebuffer.point(x, y + 1);
    framebuffer.point(x + 2, y + 1);
    framebuffer.point(x + 1, y + 2);
}

fn gosper_glider_gun(framebuffer: &mut Framebuffer, x: usize, y: usize) {
    let coordinates = [
        (x, y + 4), (x, y + 5), (x + 1, y + 4), (x + 1, y + 5),
        (x + 10, y + 4), (x + 10, y + 5), (x + 10, y + 6),
        (x + 11, y + 3), (x + 11, y + 7), (x + 12, y + 2), (x + 12, y + 8),
        (x + 13, y + 2), (x + 13, y + 8), (x + 14, y + 5),
        (x + 15, y + 3), (x + 15, y + 7), (x + 16, y + 4), (x + 16, y + 5), (x + 16, y + 6),
        (x + 17, y + 5), (x + 20, y + 2), (x + 20, y + 3), (x + 20, y + 4),
        (x + 21, y + 2), (x + 21, y + 3), (x + 21, y + 4), (x + 22, y + 1), (x + 22, y + 5),
        (x + 24, y), (x + 24, y + 1), (x + 24, y + 5), (x + 24, y + 6),
        (x + 34, y + 2), (x + 34, y + 3), (x + 35, y + 2), (x + 35, y + 3),
    ];

    for &(dx, dy) in &coordinates {
        framebuffer.point(dx, dy);
    }
}

fn pulsar(framebuffer: &mut Framebuffer, x: usize, y: usize) {
    let coordinates = [
        (x + 2, y), (x + 3, y), (x + 4, y),
        (x, y + 2), (x, y + 3), (x, y + 4),
        (x + 5, y + 2), (x + 5, y + 3), (x + 5, y + 4),
        (x + 2, y + 5), (x + 3, y + 5), (x + 4, y + 5),
        
        (x + 8, y), (x + 9, y), (x + 10, y),
        (x + 7, y + 2), (x + 7, y + 3), (x + 7, y + 4),
        (x + 12, y + 2), (x + 12, y + 3), (x + 12, y + 4),
        (x + 8, y + 5), (x + 9, y + 5), (x + 10, y + 5),

        (x + 2, y + 7), (x + 3, y + 7), (x + 4, y + 7),
        (x, y + 8), (x, y + 9), (x, y + 10),
        (x + 5, y + 8), (x + 5, y + 9), (x + 5, y + 10),
        (x + 2, y + 12), (x + 3, y + 12), (x + 4, y + 12),

        (x + 8, y + 7), (x + 9, y + 7), (x + 10, y + 7),
        (x + 7, y + 8), (x + 7, y + 9), (x + 7, y + 10),
        (x + 12, y + 8), (x + 12, y + 9), (x + 12, y + 10),
        (x + 8, y + 12), (x + 9, y + 12), (x + 10, y + 12),
    ];

    for &(dx, dy) in &coordinates {
        framebuffer.point(dx, dy);
    }
}

fn penta_decathlon(framebuffer: &mut Framebuffer, x: usize, y: usize) {
    let coordinates = [
        (x + 2, y), (x + 3, y), (x + 4, y),
        (x + 1, y + 1), (x + 5, y + 1),
        (x + 2, y + 2), (x + 3, y + 2), (x + 4, y + 2),
        (x + 2, y + 4), (x + 3, y + 4), (x + 4, y + 4),
        (x + 1, y + 5), (x + 5, y + 5),
        (x + 2, y + 6), (x + 3, y + 6), (x + 4, y + 6),
        (x + 2, y + 8), (x + 3, y + 8), (x + 4, y + 8),
    ];

    for &(dx, dy) in &coordinates {
        framebuffer.point(dx, dy);
    }
}
fn schick_engine(framebuffer: &mut Framebuffer, x: usize, y: usize) {
    let coordinates = [
        (x, y), (x, y + 1), (x, y + 2), (x, y + 3), (x, y + 4), (x, y + 5), (x, y + 6), (x, y + 7), (x, y + 8),
        (x + 1, y), (x + 2, y), (x + 3, y), (x + 4, y), (x + 5, y), (x + 6, y), (x + 7, y), (x + 8, y),
        (x + 3, y + 3), (x + 3, y + 4), (x + 4, y + 4),
        (x + 4, y + 6), (x + 5, y + 6), (x + 5, y + 5), (x + 6, y + 5), (x + 6, y + 4),
    ];

    for &(dx, dy) in &coordinates {
        framebuffer.point(dx, dy);
    }
}

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

    framebuffer.set_background_color(Color::darkblue());
    framebuffer.set_current_color(Color::white());

    initial_pattern(&mut framebuffer);

    let frame_delay = Duration::from_millis(100);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        update_game(&mut framebuffer);

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}