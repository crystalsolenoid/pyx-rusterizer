use crate::buffer::Buffer;

pub struct Tri {
    pub v1: (f32, f32),
    pub v2: (f32, f32),
    pub v3: (f32, f32),
}

pub fn draw_tri(buffer: &mut Buffer, tri: Tri, color: u8) {
    let split_triange = SplitTriangle::new(tri);

    let tri_x = 10;
    let tri_y = 10;
    buffer.h_line(3 + tri_x, 5 + tri_x, 5 + tri_y, color);
    buffer.h_line(2 + tri_x, 6 + tri_x, 6 + tri_y, color);
    buffer.h_line(2 + tri_x, 7 + tri_x, 7 + tri_y, color);
    buffer.h_line(1 + tri_x, 8 + tri_x, 8 + tri_y, color);
}

struct UpDownTri {
    tip_x: f32,
    tip_y: f32,
    base_y: f32,
    base_left_x: f32,
    base_right_x: f32,
}

impl UpDownTri {
    fn new(base_1: (f32, f32), base_2: (f32, f32), tip: (f32, f32)) -> Self {
        UpDownTri {
            tip_x: tip.0,
            tip_y: tip.1,
            base_y: base_1.1,
            base_left_x: base_1.0.min(base_2.0),
            base_right_x: base_1.0.max(base_2.0),
        }
    }
    fn draw(self, buffer: &mut Buffer, color: u8) {}
    fn draw_up(self, buffer: &mut Buffer, color: u8) {}
    fn draw_down(self, buffer: &mut Buffer, color: u8) {}
}

struct SplitTriangle {
    up_tri: Option<UpDownTri>,
    down_tri: Option<UpDownTri>,
}

impl SplitTriangle {
    fn new(tri: Tri) -> Self {
        let mut points = [tri.v1, tri.v2, tri.v3];
        points.sort_by(|t1, t2| t1.1.partial_cmp(&t2.1).unwrap());
        let bot_point = points[0];
        let mid_point = points[1];
        let top_point = points[2];

        // check for a horizontal straight line
        if bot_point.1 == mid_point.1 && mid_point.1 == top_point.1 {
            return SplitTriangle {
                up_tri: None,
                down_tri: None,
            };
        };

        // check if already up
        if bot_point.1 == mid_point.1 {
            return SplitTriangle {
                up_tri: Some(UpDownTri::new(bot_point, mid_point, top_point)),
                down_tri: None,
            };
        };
        // check if already down
        if top_point.1 == mid_point.1 {
            return SplitTriangle {
                up_tri: None,
                down_tri: Some(UpDownTri::new(top_point, mid_point, bot_point)),
            };
        };

        let new_base_y = mid_point.1;
        let new_base_x = lerp(top_point, bot_point, new_base_y);

        SplitTriangle {
            up_tri: Some(UpDownTri::new(
                mid_point,
                (new_base_x, new_base_y),
                top_point,
            )),
            down_tri: Some(UpDownTri::new(
                mid_point,
                (new_base_x, new_base_y),
                bot_point,
            )),
        }
    }
    fn draw(self, buffer: &mut Buffer, color: u8) {
        match self.up_tri {
            Some(tri) => tri.draw(buffer, color),
            None => (),
        }
        match self.down_tri {
            Some(tri) => tri.draw(buffer, color),
            None => (),
        }
    }
}
fn lerp(p1: (f32, f32), p2: (f32, f32), y: f32) -> f32 {
    let x1 = p1.0;
    let y1 = p1.1;
    let x2 = p2.0;
    let y2 = p2.1;

    let x = (x2 - x1) / (y2 - y1) * (y - y1) + x1;
    x
}
