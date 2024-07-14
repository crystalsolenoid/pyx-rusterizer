use std::{cmp, fmt::Pointer};

pub struct Buffer {
    pub pixels: Vec<u8>,
    pub rgb_pixels: Vec<u32>,
    pub width: usize,
    pub height: usize,
    pub scale: usize,
    pub palette: [u32; 4],
}

impl Buffer {
    //    pub fn new
    pub fn pix(&mut self, x: usize, y: usize, color: u8) {
        self.pixels[y * self.width + x] = color;
        let rgb_color = self.palette[color as usize];

        for row in y * self.scale..(y + 1) * self.scale {
            let index = row * self.width * self.scale + (x * self.scale);
            let slice = &mut self.rgb_pixels[index..index + self.scale];
            slice.fill(rgb_color);
        }
    }

    //TODO change usize to i32, and check y bounds
    pub fn h_line(&mut self, x1: usize, x2: usize, y: usize, color: u8) {
        let start = cmp::max(x1, 0);
        let end = cmp::min(x2, self.width);

        let offset = y * self.width;
        self.pixels[offset + start..offset + end].fill(color);

        let rgb_color = self.palette[color as usize];

        for row in y * self.scale..(y + 1) * self.scale {
            let offset = row * self.width * self.scale;
            let slice =
                &mut self.rgb_pixels[offset + start * self.scale..end * self.scale + offset];
            slice.fill(rgb_color);
        }
    }
}
