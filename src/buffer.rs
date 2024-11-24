use core::f32;

use glam::Vec2;

use crate::{
    color::Color,
    interpolate::{lerp, LerpIter},
};

/// u8 value, because that is the biggest that will fit into palette_pixels
const COLOR_DEPTH: u8 = 32;
//TODO: create a type for indexed colors

/// Contains the current frames data both as both
///
/// `canvas`: unscaled, indexed colored mode
///
/// and
///
/// `rgb_pixels`: scaled up to screen resolution, rgb colors
pub struct Buffer {
    /// unscaled width
    width: usize,
    /// unscaled height
    height: usize,
    /// map color indicies to u32 rgb values
    pub palette: [u32; COLOR_DEPTH as usize],

    /// User controlled, screen buffer, holding color palette indices
    /// Length is `width * height` (not scaled by `scale`)
    canvas: Vec<u8>,
    z_buffer: Vec<f32>,

    /// API Controlled screen buffer holding u32 values that represent rgb values
    /// Length is `(width  * scale) * (height * scale)`
    /// Always kept in sync with `canvas`
    rgb_pixels: Vec<u32>,
    scale: usize,
}

impl Buffer {
    //    pub fn new
    pub fn new(
        width: usize,
        height: usize,
        palette: [u32; COLOR_DEPTH as usize],
        scale: usize,
    ) -> Self {
        Buffer {
            width,
            height,
            palette,
            canvas: vec![0; width * height],
            z_buffer: vec![f32::NEG_INFINITY; width * height],
            rgb_pixels: vec![palette[0]; (width * scale) * (height * scale)],

            scale,
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
    pub fn rgb_pixels(&self) -> &Vec<u32> {
        &self.rgb_pixels
    }
    pub fn clear_screen(&mut self) {
        self.canvas.fill(0);
        self.z_buffer.fill(f32::NEG_INFINITY);
        self.rgb_pixels.fill(0);
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

        //update rbg_pixels
        let rgb_color = self.palette[color as usize];

        for row in y * self.scale..(y + 1) * self.scale {
            let index = row * self.width * self.scale + (x * self.scale);
            let slice = &mut self.rgb_pixels[index..index + self.scale];
            slice.fill(rgb_color);
        }
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

    pub fn h_line(&mut self, x1: f32, x2: f32, y: i32, z1: f32, z2: f32, color: u8) {
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

        let rgb_color = self.palette[color as usize];
        range.zip(z_values).for_each(|(x, (_, z))| {
            //// Z buffer test
            if z > self.z_buffer[canvas_offset + x] {
                //// Update Canvas/Z-buffer
                self.z_buffer[canvas_offset + x] = z;
                self.canvas[canvas_offset + x] = color;

                //// Update rgb_pixels
                for row in y * self.scale..(y + 1) * self.scale {
                    let offset = row * self.width * self.scale;
                    let slice = &mut self.rgb_pixels
                        [offset + x * self.scale..(x + 1) * self.scale + offset];
                    slice.fill(rgb_color);
                }
            }
        });
    }
}
