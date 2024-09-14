use glam::{Affine3A, Vec3};
use minifb::{Key, Window, WindowOptions};
use std::{
    f32::consts::PI,
    time::{Duration, Instant},
};

use pyx_rusterizer::{
    buffer::Buffer,
    geo::{Geo, Mesh, Triangle},
    poly,
};

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
    cube: Geo,
}

impl Model {
    fn new() -> Self {
        Model {
            cube: Geo::new(
                Box::new(Mesh {
                    vertices: vec![
                        //top
                        Vec3::new(-0.5, -0.5, -0.5),
                        Vec3::new(0.5, -0.5, -0.5),
                        Vec3::new(-0.5, 0.5, -0.5),
                        Vec3::new(0.5, 0.5, -0.5),
                        Vec3::new(-0.5, -0.5, 0.5),
                        Vec3::new(0.5, -0.5, 0.5),
                        Vec3::new(-0.5, 0.5, 0.5),
                        Vec3::new(0.5, 0.5, 0.5),
                    ],
                    triangles: vec![
                        //right
                        Triangle {
                            index: (1, 3, 5),
                            color: 3,
                        },
                        Triangle {
                            index: (3, 5, 7),
                            color: 3,
                        },
                        //left
                        Triangle {
                            index: (0, 2, 4),
                            color: 3,
                        },
                        Triangle {
                            index: (2, 4, 6),
                            color: 3,
                        },
                        //top
                        Triangle {
                            index: (2, 3, 6),
                            color: 1,
                        },
                        Triangle {
                            index: (3, 6, 7),
                            color: 1,
                        },
                        //bottom
                        Triangle {
                            index: (0, 1, 4),
                            color: 1,
                        },
                        Triangle {
                            index: (1, 4, 5),
                            color: 1,
                        },
                        //front
                        Triangle {
                            index: (4, 5, 6),
                            color: 2,
                        },
                        Triangle {
                            index: (5, 6, 7),
                            color: 2,
                        },
                        //back
                        Triangle {
                            index: (0, 1, 2),
                            color: 2,
                        },
                        Triangle {
                            index: (1, 2, 3),
                            color: 2,
                        },
                    ],
                }),
                Affine3A::IDENTITY,
            ),
        }
    }
}

/// called every tick
fn update(timing: Timing, model: &mut Model) {
    let t = timing.time_since_start.as_secs_f32();

    model.cube.transform = Affine3A::from_translation(Vec3::new(20., 40., 0.))
        * Affine3A::from_rotation_y(t * PI / 2.)
        * Affine3A::from_rotation_x(t * PI / 2.)
        * Affine3A::from_scale(Vec3::splat(10.));
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
