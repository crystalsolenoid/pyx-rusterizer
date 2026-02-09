use core::f32;

use glam::Vec2;
use num_traits::ToBytes;

use crate::{
    constants::COLOR_DEPTH,
    interpolate::{lerp, LerpIter},
};

//TODO: create a type for indexed colors

const CLEAR_COLOR: u8 = 21;
/// Contains the current frames data both as
///
/// `canvas`: unscaled, indexed colored mode
pub struct Buffer {
    width: usize,
    height: usize,
    /// map color indicies to u32 rgb values
    pub palette: [u32; COLOR_DEPTH as usize],

    /// User controlled, screen buffer, holding color palette indices
    /// Length is `width * height`
    pub canvas: Vec<u8>,
    z_buffer: Vec<f32>,
    // TODO this index is per-mesh, so we need to specify a mesh
    // in addition to the triangle index
    tri_buffer: Vec<Option<usize>>,
}

impl Buffer {
    //    pub fn new
    pub fn new(width: usize, height: usize, palette: [u32; COLOR_DEPTH as usize]) -> Self {
        Buffer {
            width,
            height,
            palette,
            canvas: vec![CLEAR_COLOR; width * height],
            z_buffer: vec![f32::NEG_INFINITY; width * height],
            tri_buffer: vec![None; width * height],
        }
    }

    /// Current canvas width
    pub fn width(&self) -> usize {
        self.width
    }

    /// Current canvas height
    pub fn height(&self) -> usize {
        self.height
    }

    pub fn tri_idx_at_pixel(&self, x: usize, y: usize) -> Option<usize> {
        let i = y * self.width + x;
        self.tri_buffer[i]
    }

    pub fn get_palette_rgb(&self) -> Vec<[u8; 4]> {
        self.canvas
            .iter()
            .map(|idx| ToBytes::to_be_bytes(&self.palette[*idx as usize]))
            .collect()
    }

    pub fn clear_screen(&mut self) {
        self.canvas.fill(CLEAR_COLOR);
        self.z_buffer.fill(f32::NEG_INFINITY);
        self.tri_buffer.fill(None);
    }

    /// sets an indexed color at `x`,`y`
    pub fn pix(&mut self, x: i32, y: i32, color: u8) {
        let x = Self::clamp_i32(x, 0, self.width);
        let y = Self::clamp_i32(y, 0, self.height);

        Self::pix_unchecked(self, x, y, color);
    }

    fn pix_unchecked(&mut self, x: usize, y: usize, color: u8) {
        // update pixels
        self.canvas[y * self.width + x] = color;
    }

    fn clamp_i32(x: i32, min: usize, max: usize) -> usize {
        match usize::try_from(x) {
            Ok(val) => val.clamp(min, max),
            Err(_) => {
                if x < 0 {
                    min
                } else {
                    max
                }
            }
        }
    }

    pub fn h_line(
        &mut self,
        x1: f32,
        x2: f32,
        y: i32,
        z1: f32,
        z2: f32,
        color: u8,
        tri_idx: usize,
    ) {
        let y = match usize::try_from(y) {
            Ok(val) => {
                if val >= self.height {
                    return;
                } else {
                    val
                }
            }
            Err(_) => return,
        };

        let x1_int = x1.ceil() as i32;
        let x2_int = x2.floor() as i32 + 1;

        let x_start = Self::clamp_i32(x1_int, 0, self.width);
        let x_end = Self::clamp_i32(x2_int, 0, self.width);

        let z_start = lerp(Vec2::new(z1, x1), Vec2::new(z2, x2), x_start as f32);
        let z_end = lerp(Vec2::new(z1, x1), Vec2::new(z2, x2), x_end as f32);

        let h_line_width = x_end - x_start;

        let canvas_offset = y * self.width;
        let range = x_start..x_end;

        let z_values = LerpIter::new(
            (x_start as f32, z_start),
            (x_end as f32, z_end),
            h_line_width + 1,
        );

        range.zip(z_values).for_each(|(x, (_, z))| {
            //// Z buffer test
            if z > self.z_buffer[canvas_offset + x] {
                //// Update Canvas/Z-buffer
                self.z_buffer[canvas_offset + x] = z;
                self.canvas[canvas_offset + x] = color;
                self.tri_buffer[canvas_offset + x] = Some(tri_idx);
            }
        });
    }
}
