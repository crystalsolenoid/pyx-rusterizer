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

        for row in y * self.scale..(y + 1) * self.scale {
            let index = row * self.width * self.scale + (x * self.scale);
            let slice = &mut self.rgb_pixels[index..index + self.scale];
            slice.fill(self.palette[color as usize]);
        }
    }
}
