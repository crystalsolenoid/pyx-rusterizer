use assets_manager::AssetCache;
use num_traits::ToBytes;

// TODO: use palette for background
// TODO: stop printing mesh info

use pyx_rusterizer::{
    buffer::Buffer,
    color::{NamedMaterials, Palette},
    constants::{HEIGHT, WIDTH},
    gui,
    model::Model,
};

fn main() {
    // env_logger::init(); // This is done in icecube now. Is that ok?

    let cache = AssetCache::new("assets").unwrap();
    let palette_handle = cache.load::<Palette>("palette").unwrap();
    let material_handle = cache.load::<NamedMaterials>("porygon.materials").unwrap();

    let mut buffer: Buffer;
    {
        let palette = palette_handle.read();

        buffer = Buffer::new(WIDTH, HEIGHT, palette.colors);
    }

    /*
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
    */

    let mut model = Model::new(material_handle);

    // let start_instant = Instant::now();
    // let mut last_frame_instant = Instant::now();
    // let mut timing: Timing;

    cache.hot_reload();
    buffer.palette = palette_handle.read().colors;
    //TODO: figure out how to get Materials out of the AssetReadGuard without cloning
    model.cube.shape.materials = NamedMaterials(material_handle.read().0.clone()).into();

    let initial_state = gui::State::new(buffer, model);
    icecube::run(
        initial_state,
        gui::update,
        gui::view,
        320,
        240,
        ToBytes::to_be_bytes(&palette_handle.read().colors[21]),
        |d| Some(gui::Message::TimeElapsed(d)),
    )
    .unwrap();
    /*
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
    */
}
