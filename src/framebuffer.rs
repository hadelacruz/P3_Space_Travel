use crate::shaders::ShaderColor;

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
    pub depth_buffer: Vec<f32>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            width,
            height,
            buffer: vec![0; width * height],
            depth_buffer: vec![f32::INFINITY; width * height],
        }
    }

    pub fn clear(&mut self) {
        self.buffer.fill(0);
        self.depth_buffer.fill(f32::INFINITY);
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            self.buffer[index] = color;
        }
    }

    pub fn set_pixel_with_depth(&mut self, x: usize, y: usize, depth: f32, color: u32) {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            if depth < self.depth_buffer[index] {
                self.depth_buffer[index] = depth;
                self.buffer[index] = color;
            }
        }
    }

    pub fn get_buffer(&self) -> &[u32] {
        &self.buffer
    }
}

pub fn rgb_to_u32(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

pub fn shader_color_to_u32(color: &ShaderColor) -> u32 {
    let r = (color.r.clamp(0.0, 1.0) * 255.0) as u8;
    let g = (color.g.clamp(0.0, 1.0) * 255.0) as u8;
    let b = (color.b.clamp(0.0, 1.0) * 255.0) as u8;
    rgb_to_u32(r, g, b)
}
