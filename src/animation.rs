use std::{f32::consts::PI, time::Duration};

use glam::{Affine3A, Vec3};

use crate::{
    constants::{HEIGHT, WIDTH},
    model::Model,
};

/// called every tick
pub fn update(radians: f32, model: &mut Model) {
    // let t = timing.time_since_start.as_secs_f32();

    model.cube.transform =
        Affine3A::from_translation(Vec3::new(WIDTH as f32 / 2., HEIGHT as f32 / 2., 0.))
            * Affine3A::from_rotation_x(PI + 0.01)
            * Affine3A::from_rotation_y(radians)
            * Affine3A::from_scale(Vec3::splat(50.));
}

pub struct Timing {
    pub time_since_start: Duration,
    pub _delta: f32,
}
