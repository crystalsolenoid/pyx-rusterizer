use minifb::{Key, Window, WindowOptions};
use std::time::{Duration, Instant};

use pyx_rusterizer::buffer::Buffer;

const WIDTH: usize = 40;
const HEIGHT: usize = 60;
const SCALING_FACTOR: usize = 10;

// packedRGB values, indexed by paletteIndex
const PALETTE: [u32; 4] = [
    (0 << 16) | (0 << 8) | 0,
    (50 << 16) | (50 << 8) | 50,
    (0 << 16) | (255 << 8) | 255,
    (255 << 16) | (255 << 8) | 255,
];

struct Model {
    triangle_color: u8,
    triangle_position: (usize, usize),
}

impl Model {
    fn new() -> Self {
        Model {
            triangle_color: 0,
            triangle_position: (WIDTH / 2, HEIGHT / 2),
        }
    }
}

/// called every tick
fn update(timing: Timing, model: &mut Model) {
    model.triangle_color = (timing.time_since_start.as_secs() % 4) as u8;
}

/// called every frame
fn draw(buffer: &mut Buffer, model: &Model) {
    for i in 0..buffer.width() * buffer.height() {
        buffer.pix(i % buffer.width(), i / buffer.width(), ((i / 11) % 4) as u8);
    }
    let tri_x = model.triangle_position.0 - 4;
    let tri_y = model.triangle_position.1 - 6;

    buffer.h_line(3 + tri_x, 5 + tri_x, 5 + tri_y, model.triangle_color);
    buffer.h_line(2 + tri_x, 6 + tri_x, 6 + tri_y, model.triangle_color);
    buffer.h_line(2 + tri_x, 7 + tri_x, 7 + tri_y, model.triangle_color);
    buffer.h_line(1 + tri_x, 8 + tri_x, 8 + tri_y, model.triangle_color);
}

struct Timing {
    time_since_start: Duration,
    delta: f32,
}

fn main() {
    let mut buffer = Buffer::new(WIDTH, HEIGHT, PALETTE, SCALING_FACTOR);

    let mut window = Window::new(
        "Test - ESC to exit",
        SCALING_FACTOR * buffer.width(),
        SCALING_FACTOR * buffer.height(),
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.set_target_fps(60);

    let mut model = Model::new();

    let start_instant = Instant::now();
    let mut last_frame_instant = Instant::now();
    let mut timing;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        draw(&mut buffer, &model);

        timing = Timing {
            time_since_start: Instant::now() - start_instant,
            delta: (Instant::now() - last_frame_instant).as_secs_f32(),
        };
        last_frame_instant = Instant::now();

        update(timing, &mut model);

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(
                buffer.rgb_pixels(),
                SCALING_FACTOR * buffer.width(),
                SCALING_FACTOR * buffer.height(),
            )
            .unwrap();
    }
}
