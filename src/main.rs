use assets_manager::{AssetCache, Handle};
use glam::{Affine3A, Vec3};
use minifb::{Key, Window, WindowOptions};
use std::{
    f32::consts::PI,
    path::Path,
    time::{Duration, Instant},
};

// TODO: use palette for background
// TODO: stop printing mesh info

use pyx_rusterizer::{
    buffer::Buffer,
    color::{NamedMaterials, Palette},
    geo::Geo,
    obj,
};

const WIDTH: usize = 80;
const HEIGHT: usize = 120;
const SCALING_FACTOR: usize = 5;

struct Model {
    cube: Geo,
}

impl Model {
    fn new(material_handle: &Handle<NamedMaterials>) -> Model {
        let named_materials: NamedMaterials = NamedMaterials(material_handle.read().0.clone());

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
    env_logger::init();

    let cache = AssetCache::new("assets").unwrap();
    let palette_handle = cache.load::<Palette>("palette").unwrap();
    let material_handle = cache.load::<NamedMaterials>("porygon.materials").unwrap();

    let mut buffer: Buffer;
    {
        let palette = palette_handle.read();

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

    let mut model = Model::new(material_handle);

    let start_instant = Instant::now();
    let mut last_frame_instant = Instant::now();
    let mut timing;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        cache.hot_reload();
        buffer.palette = palette_handle.read().colors;
        //TODO: figure out how to get Materials out of the AssetReadGuard without cloning
        model.cube.shape.materials = NamedMaterials(material_handle.read().0.clone()).into();

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
