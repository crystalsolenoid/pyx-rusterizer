use glam::{Affine3A, Vec3};
use minifb::{Key, Window, WindowOptions};
use serde::Deserialize;
use std::{
    f32::consts::PI,
    fs::read_to_string,
    path::Path,
    time::{Duration, Instant},
};
use toml;

use pyx_rusterizer::{buffer::Buffer, geo::Geo, obj};

const WIDTH: usize = 80;
const HEIGHT: usize = 120;
const SCALING_FACTOR: usize = 5;

#[derive(Deserialize, Debug)]
struct Palette {
    colors: [u32; 32],
}

struct Model {
    cube: Geo,
}

impl Model {
    fn new() -> Self {
        let obj = obj::parse(Path::new("assets/porygon/model.obj")).unwrap();
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
            * Affine3A::from_rotation_x(PI + 0.01)
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
    let pal_path = Path::new("assets/palette.toml");
    let palette_string = read_to_string(pal_path).unwrap();
    let palette: Palette = toml::from_str(&palette_string).expect("deserialization failed");

    let mut buffer = Buffer::new(WIDTH, HEIGHT, palette.colors, SCALING_FACTOR);

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
