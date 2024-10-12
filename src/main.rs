use glam::{Affine3A, Vec3};
use minifb::{Key, Window, WindowOptions};
use std::{
    f32::consts::PI,
    path::Path,
    time::{Duration, Instant},
};

use pyx_rusterizer::{buffer::Buffer, geo::Geo, obj};

const WIDTH: usize = 80;
const HEIGHT: usize = 120;
const SCALING_FACTOR: usize = 5;

// packedRGB values, indexed by paletteIndex
const PALETTE: [u32; 32] = [
    0xFF000000, 0xFF00021c, 0xFF1c284d, 0xFF343473, 0xFF2d5280, 0xFF4d7a99, 0xFF7497a6, 0xFFa3ccd9,
    0xFFf0edd8, 0xFF732866, 0xFFa6216e, 0xFFd94c87, 0xFFd9214f, 0xFFf25565, 0xFFf27961, 0xFF993649,
    0xFFb36159, 0xFFf09c60, 0xFFb38f24, 0xFFb3b324, 0xFFf7c93e, 0xFF17735f, 0xFF119955, 0xFF67b31b,
    0xFF1ba683, 0xFF47cca9, 0xFF96e3c9, 0xFF2469b3, 0xFF0b8be6, 0xFF0bafe6, 0xFFf28d85, 0xFFf0bb90,
];

struct Model {
    cube: Geo,
}

impl Model {
    fn new() -> Self {
        let obj = obj::parse(Path::new("assets/porygon/model.obj"));
        println!("{:?}", obj);
        Model {
            cube: Geo::new(Box::new(obj), Affine3A::IDENTITY),
        }
    }
}

/// called every tick
fn update(timing: Timing, model: &mut Model) {
    let t = timing.time_since_start.as_secs_f32();

    model.cube.transform =
        Affine3A::from_translation(Vec3::new(WIDTH as f32 / 2., HEIGHT as f32 / 2., 0.))
            * Affine3A::from_rotation_x(PI)
            * Affine3A::from_rotation_y(-t * PI / 3.)
            * Affine3A::from_scale(Vec3::splat(50.));
}

/// called every frame
fn draw(buffer: &mut Buffer, model: &Model) {
    buffer.clear_screen();

    model.cube.render(buffer);
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
