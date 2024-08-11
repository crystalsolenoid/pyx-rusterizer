use std::cmp;

/// u8 value, because that is the biggest that will fit into palette_pixels
const COLOR_DEPTH: u8 = 4;

pub struct Buffer {
    /// unscaled width
    width: usize,
    /// unscaled height
    height: usize,
    /// map color indicies to u32 rgb values
    palette: [u32; COLOR_DEPTH as usize],

    /// User controlled, screen buffer, holding color palette indices
    /// Length is `width * height` (not scaled by `scale`)
    canvas: Vec<u8>,

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

    /// sets a
    pub fn pix(&mut self, x: usize, y: usize, color: u8) {
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

    //TODO change usize to i32, and check y bounds
    pub fn h_line(&mut self, x1: usize, x2: usize, y: usize, color: u8) {
        let start = cmp::max(x1, 0);
        let end = cmp::min(x2, self.width);

        let offset = y * self.width;
        self.canvas[offset + start..offset + end].fill(color);

        let rgb_color = self.palette[color as usize];

        for row in y * self.scale..(y + 1) * self.scale {
            let offset = row * self.width * self.scale;
            let slice =
                &mut self.rgb_pixels[offset + start * self.scale..end * self.scale + offset];
            slice.fill(rgb_color);
        }
    }
}
