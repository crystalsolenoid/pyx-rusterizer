use assets_manager::AssetCache;
use glam::{Affine3A, Vec3};
use minifb::{Key, Window, WindowOptions};
use serde::{Deserialize, Serialize};
use std::{
    f32::consts::PI,
    fs::read_to_string,
    path::Path,
    time::{Duration, Instant},
};
use toml;

// TODO: use palette for background
// TODO: stop printing mesh info

use pyx_rusterizer::{
    buffer::Buffer,
    color::{Material, NamedMaterials, Palette},
    geo::Geo,
    obj,
};

const WIDTH: usize = 80;
const HEIGHT: usize = 120;
const SCALING_FACTOR: usize = 5;

struct Scene {
    palette: Palette,
}

struct Model {
    cube: Geo,
}

impl Model {
    fn new() -> Model {
        let mat_path = Path::new("assets/porygon/materials.toml");
        let mat_string = read_to_string(mat_path).unwrap();
        let named_materials: NamedMaterials =
            toml::from_str(&mat_string).expect("deserialization failed");

        let mesh = obj::parse(Path::new("assets/porygon/model.obj"), named_materials).unwrap();
        println!("{:?}", mesh);
        Model {
            cube: Geo::new(mesh, Affine3A::IDENTITY),
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
    let cache = AssetCache::new("assets").unwrap();
    let handle = cache.load::<Palette>("palette").unwrap();

    let mut buffer: Buffer;
    {
        let palette = handle.read();

        buffer = Buffer::new(WIDTH, HEIGHT, palette.colors, SCALING_FACTOR);
    }

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
        cache.hot_reload();
        println!("Current value: {:?}", handle.read());
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
