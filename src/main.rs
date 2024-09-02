use minifb::{Key, Window, WindowOptions};
use std::time::{Duration, Instant};

use pyx_rusterizer::{buffer::Buffer, poly};

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
    triangle_position: (i32, i32),
}

impl Model {
    fn new() -> Self {
        Model {
            triangle_color: 0,
            triangle_position: (WIDTH as i32 / 2, HEIGHT as i32 / 2),
        }
    }
}

/// called every tick
fn update(timing: Timing, model: &mut Model) {
    model.triangle_color = (timing.time_since_start.as_secs() % 4) as u8;

    let t = timing.time_since_start.as_secs_f32();
    model.triangle_position.0 = (WIDTH as f32 / 1. * (t.sin() + 1.)) as i32;
    model.triangle_position.1 = (HEIGHT as f32 / 1. * (t.cos() + 1.)) as i32;
}

/// called every frame
fn draw(buffer: &mut Buffer, _model: &Model) {
    //for i in 0..(buffer.width() * buffer.height()) as i32 {
    //    buffer.pix(
    //        i % buffer.width() as i32,
    //        i / buffer.width() as i32,
    //        ((i / 11) % 4) as u8,
    //    );
    //}
    //let tri_x = model.triangle_position.0 - 4;
    //let tri_y = model.triangle_position.1 - 6;

    let v1 = (20., 5.);
    let v2 = (5., 55.);
    let v3 = (35., 55.);
    let v4 = (20., 35.);

    let tri1 = poly::Tri {
        v1: v1,
        v2: v2,
        v3: v4,
    };
    let tri2 = poly::Tri {
        v1: v1,
        v2: v3,
        v3: v4,
    };
    let tri3 = poly::Tri {
        v1: v2,
        v2: v3,
        v3: v4,
    };

    //buffer.pix(tri.v1.0 as i32, tri.v1.1 as i32, 3);
    //buffer.pix(tri.v2.0 as i32, tri.v2.1 as i32, 3);
    //buffer.pix(tri.v3.0 as i32, tri.v3.1 as i32, 3);
    poly::draw_tri(buffer, &tri3, 3);
    poly::draw_tri(buffer, &tri1, 1);
    poly::draw_tri(buffer, &tri2, 2);
}

struct Timing {
    time_since_start: Duration,
    _delta: f32,
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
            _delta: (Instant::now() - last_frame_instant).as_secs_f32(),
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
