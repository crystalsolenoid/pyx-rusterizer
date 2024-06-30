use minifb::{Key, Window, WindowOptions};
use std::time::Instant;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

struct Buffer {
    //vector of paletteIndicies
    pixels: Vec<u8>,
    width: usize,
    height: usize,
}

// packedRGB values, indexed by paletteIndex
const PALETTE: [u32; 4] = [
    (0 << 16) | (0 << 8) | 0,
    (50 << 16) | (50 << 8) | 50,
    (0 << 16) | (255 << 8) | 255,
    (255 << 16) | (255 << 8) | 255,
];

fn main() {
    let mut palette_buffer = Buffer {
        pixels: vec![1; WIDTH * HEIGHT],
        width: WIDTH,
        height: HEIGHT,
    };

    let mut rgb_buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
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
            for i in palette_buffer.pixels.iter_mut() {
                *i = frames % 4;
            }

            rgb_buffer = palette_buffer
                .pixels
                .iter()
                .map(|&palette_index| PALETTE[palette_index as usize])
                .collect();

            now = Instant::now();
            frames += 1;
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&rgb_buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}