use minifb::{Key, Window, WindowOptions};
use std::time::Instant;

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

fn main() {
    let mut palette_buffer = Buffer {
        pixels: vec![0; WIDTH * HEIGHT],
        rgb_pixels: vec![0; WIDTH * HEIGHT * SCALING_FACTOR * SCALING_FACTOR],
        width: WIDTH,
        height: HEIGHT,
        scale: SCALING_FACTOR,
        palette: PALETTE,
    };

    for i in 0..palette_buffer.width * palette_buffer.height {
        palette_buffer.pix(
            i % palette_buffer.width,
            i / palette_buffer.width,
            ((i / 11) % 4) as u8,
        );
    }

    let mut window = Window::new(
        "Test - ESC to exit",
        SCALING_FACTOR * palette_buffer.width,
        SCALING_FACTOR * palette_buffer.height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.set_target_fps(60);

    let mut frames = 0;
    let mut now = Instant::now();
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let elapsed_time = now.elapsed();
        if elapsed_time.as_secs() >= 1 {
            palette_buffer.h_line(3, 5, 5, frames % 4);
            palette_buffer.h_line(2, 6, 6, frames % 4);
            palette_buffer.h_line(2, 7, 7, frames % 4);
            palette_buffer.h_line(1, 8, 8, frames % 4);

            now = Instant::now();
            frames += 1;
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(
                &palette_buffer.rgb_pixels,
                SCALING_FACTOR * palette_buffer.width,
                SCALING_FACTOR * palette_buffer.height,
            )
            .unwrap();
    }
}
