use std::cmp::Ordering;

use crate::{
    buffer::Buffer,
    color::{lit_color, Material, Palette},
    interpolate::lerp,
};
use glam::{f32::Vec3, Vec3Swizzles};

pub struct Tri {
    pub v1: Vec3,
    pub v2: Vec3,
    pub v3: Vec3,
    pub base_color: Material,
    pub illumination: f32,
}
impl Tri {
    pub fn new(v1: Vec3, v2: Vec3, v3: Vec3, base_color: Material) -> Tri {
        let diffuse_light = 0.08;
        let light_pos = Vec3::new(1000.0, -1000.0, 500.0);
        let tri_pos = (v1 + v2 + v3) / 3.;
        let cross_1 = v1 - v2;
        let cross_2 = v3 - v1;
        let normal = cross_1.cross(cross_2).normalize();
        let light_to_tri = (tri_pos - light_pos).normalize();
        let illumination = (normal.dot(light_to_tri).clamp(0.0, 1.0)) + diffuse_light;

        Tri {
            v1,
            v2,
            v3,
            base_color,
            illumination,
        }
    }
}

pub fn draw_tri(buffer: &mut Buffer, tri: &Tri) {
    let (up_tri, down_tri) = split_tri(tri);
    if let Some(up_tri) = up_tri {
        up_tri.draw_up(buffer);
    };
    if let Some(down_tri) = down_tri {
        down_tri.draw_down(buffer);
    };
    /*
    let split_triange = SplitTriangle::new(tri);
    split_triange.draw(buffer);
    */
}

#[derive(Copy, Clone)]
struct UpDownTri {
    tip: Vec3,
    base_left: Vec3,
    base_right: Vec3,
    base_color: Material,
    illumination: f32,
}

impl UpDownTri {
    fn new(
        base_1: Vec3,
        base_2: Vec3,
        tip: Vec3,
        base_color: Material,
        illumination: f32,
    ) -> UpDownTri {
        // TODO WHY?? lifetimes
        assert_eq!(base_1.y, base_2.y);
        let (base_left, base_right) = match base_1.x.partial_cmp(&base_2.x) {
            Some(Ordering::Greater) => (base_2, base_1),
            _ => (base_1, base_2),
        };

        UpDownTri {
            tip,
            base_left,
            base_right,
            base_color,
            illumination,
        }
    }

    fn draw_up(self, buffer: &mut Buffer) {
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
                lit_color(self.illumination, self.base_color),
            )
        });
    }
    fn draw_down(self, buffer: &mut Buffer) {
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
                lit_color(self.illumination, self.base_color),
            )
        });
    }
}

//struct SplitTriangle<'a> {
//    up_tri: Option<UpDownTri<'a>>,
//    down_tri: Option<UpDownTri<'a>>,
//}
//
//impl SplitTriangle<'_> {
//*/
fn split_tri<'a>(tri: &'a Tri) -> (Option<UpDownTri>, Option<UpDownTri>) {
    let mut points = [tri.v1, tri.v2, tri.v3];
    points.sort_by(|t1, t2| t1.y.partial_cmp(&t2.y).unwrap());
    let top_point = points[0];
    let mid_point = points[1];
    let bot_point = points[2];

    // check for a horizontal straight line
    if bot_point.y == mid_point.y && mid_point.y == top_point.y {
        return (None, None);
    };

    // check if already up
    if bot_point.y == mid_point.y {
        return (
            Some(UpDownTri::new(
                bot_point,
                mid_point,
                top_point,
                tri.base_color,
                tri.illumination,
            )),
            None,
        );
    };
    // check if already down
    if top_point.y == mid_point.y {
        return (
            None,
            Some(UpDownTri::new(
                top_point,
                mid_point,
                bot_point,
                tri.base_color,
                tri.illumination,
            )),
        );
    };

    let new_base_y = mid_point.y;
    let new_base_x = lerp(top_point.xy(), bot_point.xy(), new_base_y);
    let new_base_z = lerp(top_point.zy(), bot_point.zy(), new_base_y);

    let up_tri = Some(UpDownTri::new(
        mid_point,
        Vec3::new(new_base_x, new_base_y, new_base_z),
        top_point,
        tri.base_color,
        tri.illumination,
    ));
    let down_tri = Some(UpDownTri::new(
        mid_point,
        Vec3::new(new_base_x, new_base_y, new_base_z),
        bot_point,
        tri.base_color,
        tri.illumination,
    ));
    (up_tri, down_tri)
}
/*
    fn draw(self, buffer: &mut Buffer) {
        match self.up_tri {
            Some(tri) => tri.draw_up(buffer),
            None => (),
        }
        match self.down_tri {
            Some(tri) => tri.draw_down(buffer),
            None => (),
        }
    }
}
*/
