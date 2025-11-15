pub struct Framebuffer {
    pub buffer: Vec<u32>,
    pub depth_buffer: Vec<f32>,
}


impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        let space_color = rgb_to_u32(5, 10, 30);
        Framebuffer {
            buffer: vec![space_color; width * height],
            depth_buffer: vec![f32::INFINITY; width * height],
        }
    }

    pub fn clear(&mut self) {
        let space_color = rgb_to_u32(5, 10, 30);
        self.buffer.fill(space_color);
        self.depth_buffer.fill(f32::INFINITY);
    }

    pub fn get_buffer(&self) -> &[u32] {
        &self.buffer
    }
}

pub fn rgb_to_u32(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}
