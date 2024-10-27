use std::cmp::Ordering;

use crate::{
    buffer::Buffer,
    color::{grayscale, Color},
    interpolate::lerp,
};
use glam::{f32::Vec3, Vec3Swizzles};

pub struct Tri {
    pub v1: Vec3,
    pub v2: Vec3,
    pub v3: Vec3,
}

pub fn draw_tri(buffer: &mut Buffer, tri: &Tri, color: Color) {
    let split_triange = SplitTriangle::new(tri);
    split_triange.draw(buffer, color);
}

struct UpDownTri {
    tip: Vec3,
    base_left: Vec3,
    base_right: Vec3,
}

impl UpDownTri {
    fn new(base_1: Vec3, base_2: Vec3, tip: Vec3) -> Self {
        assert_eq!(base_1.y, base_2.y);
        let (base_left, base_right) = match base_1.x.partial_cmp(&base_2.x) {
            Some(Ordering::Greater) => (base_2, base_1),
            _ => (base_1, base_2),
        };

        UpDownTri {
            tip,
            base_left,
            base_right,
        }
    }

    fn draw_up(self, buffer: &mut Buffer, color: Color) {
        let base_y = self.base_left.y.floor() as i32;
        let tip_y = self.tip.y.ceil() as i32;
        let base_left = self.base_left;
        let base_right = self.base_right;
        let tip = self.tip;

        (tip_y..=base_y).for_each(|y| {
            let x_next_left = lerp(base_left.xy(), tip.xy(), y as f32);
            let x_next_right = lerp(base_right.xy(), tip.xy(), y as f32);
            let z_next_left = lerp(base_left.zy(), tip.zy(), y as f32);
            let z_next_right = lerp(base_right.zy(), tip.zy(), y as f32);

            buffer.h_line(
                x_next_left,
                x_next_right,
                y,
                z_next_left,
                z_next_right,
                color,
            )
        });
    }
    fn draw_down(self, buffer: &mut Buffer, color: Color) {
        let base_y = self.base_left.y.ceil() as i32;
        let tip_y = self.tip.y.floor() as i32;
        let base_left = self.base_left;
        let base_right = self.base_right;
        let tip = self.tip;

        (base_y..=tip_y).for_each(|y| {
            let x_next_left = lerp(base_left.xy(), tip.xy(), y as f32);
            let x_next_right = lerp(base_right.xy(), tip.xy(), y as f32);
            let z_next_left = lerp(base_left.zy(), tip.zy(), y as f32);
            let z_next_right = lerp(base_right.zy(), tip.zy(), y as f32);

            buffer.h_line(
                x_next_left,
                x_next_right,
                y,
                z_next_left,
                z_next_right,
                color,
            )
        });
    }
}

struct SplitTriangle {
    up_tri: Option<UpDownTri>,
    down_tri: Option<UpDownTri>,
}

impl SplitTriangle {
    fn new(tri: &Tri) -> Self {
        let mut points = [tri.v1, tri.v2, tri.v3];
        points.sort_by(|t1, t2| t1.y.partial_cmp(&t2.y).unwrap());
        let top_point = points[0];
        let mid_point = points[1];
        let bot_point = points[2];

        // check for a horizontal straight line
        if bot_point.y == mid_point.y && mid_point.y == top_point.y {
            return SplitTriangle {
                up_tri: None,
                down_tri: None,
            };
        };

        // check if already up
        if bot_point.y == mid_point.y {
            return SplitTriangle {
                up_tri: Some(UpDownTri::new(bot_point, mid_point, top_point)),
                down_tri: None,
            };
        };
        // check if already down
        if top_point.y == mid_point.y {
            return SplitTriangle {
                up_tri: None,
                down_tri: Some(UpDownTri::new(top_point, mid_point, bot_point)),
            };
        };

        let new_base_y = mid_point.y;
        let new_base_x = lerp(top_point.xy(), bot_point.xy(), new_base_y);
        let new_base_z = lerp(top_point.zy(), bot_point.zy(), new_base_y);

        SplitTriangle {
            up_tri: Some(UpDownTri::new(
                mid_point,
                Vec3::new(new_base_x, new_base_y, new_base_z),
                top_point,
            )),
            down_tri: Some(UpDownTri::new(
                mid_point,
                Vec3::new(new_base_x, new_base_y, new_base_z),
                bot_point,
            )),
        }
    }
    fn draw(self, buffer: &mut Buffer, color: Color) {
        let diffuse_light = 0.2;
        match self.up_tri {
            Some(tri) => {
                let light_pos = Vec3::new(100., 0.0, 0.0);
                let tri_pos = (tri.tip + tri.base_right + tri.base_left) / 3.;
                let v1 = tri.tip - tri.base_left;
                let v2 = tri.tip - tri.base_right;
                let normal = v1.cross(v2).normalize();
                let light_to_tri = (tri_pos - light_pos).normalize();
                let light_amount = (normal.dot(light_to_tri).clamp(0.0, 1.0)) + diffuse_light;

                tri.draw_up(buffer, grayscale(light_amount))
            }
            None => (),
        }
        match self.down_tri {
            Some(tri) => {
                let light_pos = Vec3::new(100., 0.0, 0.0);
                let tri_pos = (tri.tip + tri.base_right + tri.base_left) / 3.;
                let v1 = tri.tip - tri.base_left;
                let v2 = tri.tip - tri.base_right;
                let normal = -v1.cross(v2).normalize();
                let light_to_tri = (tri_pos - light_pos).normalize();
                let light_amount = (normal.dot(light_to_tri).clamp(0.0, 1.0)) + diffuse_light;

                tri.draw_down(buffer, grayscale(light_amount))
            }
            None => (),
        }
    }
}
