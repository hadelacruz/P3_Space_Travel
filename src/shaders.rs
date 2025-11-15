use crate::vector::Vector3;

#[derive(Debug, Clone, Copy)]
pub struct ShaderColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl ShaderColor {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        ShaderColor { r, g, b, a }
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        ShaderColor {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: 1.0,
        }
    }

    pub const WHITE: ShaderColor = ShaderColor { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const BLACK: ShaderColor = ShaderColor { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const YELLOW: ShaderColor = ShaderColor { r: 1.0, g: 1.0, b: 0.0, a: 1.0 };
}

pub struct ShaderUniforms {
    pub time: f32,
    pub light_direction: Vector3,
    pub camera_position: Vector3,
}

pub trait PlanetShader {
    fn vertex_shader(&self, position: Vector3, normal: Vector3, uv: (f32, f32), uniforms: &ShaderUniforms) -> (Vector3, Vector3);
    fn fragment_shader(&self, position: Vector3, normal: Vector3, uv: (f32, f32), uniforms: &ShaderUniforms) -> ShaderColor;
}


// Función de ruido Perlin
pub fn perlin_noise(x: f32, y: f32, z: f32) -> f32 {
    let xi = x.floor() as i32;
    let yi = y.floor() as i32;
    let zi = z.floor() as i32;
    
    let xf = x - x.floor();
    let yf = y - y.floor();
    let zf = z - z.floor();
    
    let u = xf * xf * (3.0 - 2.0 * xf);
    let v = yf * yf * (3.0 - 2.0 * yf);
    let w = zf * zf * (3.0 - 2.0 * zf);
    
    let hash = |x: i32, y: i32, z: i32| -> f32 {
        let n = x.wrapping_add(y.wrapping_mul(57)).wrapping_add(z.wrapping_mul(113));
        let n = (n << 13) ^ n;
        let nn = n.wrapping_mul(n.wrapping_mul(n.wrapping_mul(15731).wrapping_add(789221)).wrapping_add(1376312589));
        1.0 - ((nn & 0x7fffffff) as f32 / 1073741824.0)
    };
    
    let aaa = hash(xi, yi, zi);
    let aba = hash(xi, yi + 1, zi);
    let aab = hash(xi, yi, zi + 1);
    let abb = hash(xi, yi + 1, zi + 1);
    let baa = hash(xi + 1, yi, zi);
    let bba = hash(xi + 1, yi + 1, zi);
    let bab = hash(xi + 1, yi, zi + 1);
    let bbb = hash(xi + 1, yi + 1, zi + 1);
    
    let x1 = mix(aaa, baa, u);
    let x2 = mix(aba, bba, u);
    let x3 = mix(aab, bab, u);
    let x4 = mix(abb, bbb, u);
    
    let y1 = mix(x1, x2, v);
    let y2 = mix(x3, x4, v);
    
    mix(y1, y2, w)
}

// Funciones de ruido mejoradas para efectos procedurales
pub fn simple_noise(x: f32, y: f32) -> f32 {
    let seed = ((x * 12.9898 + y * 78.233) * 43758.5453).sin().abs();
    (seed * 1000.0).fract()
}

pub fn fbm(mut x: f32, mut y: f32, octaves: i32) -> f32 {
    let mut value = 0.0;
    let mut amplitude = 0.5;
    
    for _ in 0..octaves {
        value += amplitude * simple_noise(x, y);
        x *= 2.0;
        y *= 2.0;
        amplitude *= 0.5;
    }
    
    value
}

// Función de ruido
pub fn fbm3d(x: f32, y: f32, z: f32, octaves: i32) -> f32 {
    let mut value = 0.0;
    let mut amplitude = 0.5;
    let mut frequency = 1.0;
    
    for _ in 0..octaves {
        value += amplitude * perlin_noise(x * frequency, y * frequency, z * frequency);
        frequency *= 2.0;
        amplitude *= 0.5;
    }
    
    value
}

pub fn voronoi_noise(x: f32, y: f32) -> f32 {
    let cell_x = x.floor();
    let cell_y = y.floor();
    let mut min_dist = f32::INFINITY;
    
    for i in -1..=1 {
        for j in -1..=1 {
            let neighbor_x = cell_x + i as f32;
            let neighbor_y = cell_y + j as f32;
            let point_x = neighbor_x + simple_noise(neighbor_x, neighbor_y);
            let point_y = neighbor_y + simple_noise(neighbor_y, neighbor_x);
            
            let dist = ((x - point_x).powi(2) + (y - point_y).powi(2)).sqrt();
            min_dist = min_dist.min(dist);
        }
    }
    
    min_dist
}

pub fn ridge_noise(x: f32, y: f32, octaves: i32) -> f32 {
    let mut value = 0.0;
    let mut amplitude = 0.5;
    let mut frequency = 1.0;
    
    for _ in 0..octaves {
        let n = simple_noise(x * frequency, y * frequency);
        let ridge = 1.0 - (2.0 * n - 1.0).abs();
        value += ridge * amplitude;
        frequency *= 2.0;
        amplitude *= 0.5;
    }
    
    value
}

pub fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

pub fn mix(a: f32, b: f32, t: f32) -> f32 {
    a * (1.0 - t) + b * t
}

pub fn mix_color(a: ShaderColor, b: ShaderColor, t: f32) -> ShaderColor {
    ShaderColor::new(
        mix(a.r, b.r, t),
        mix(a.g, b.g, t),
        mix(a.b, b.b, t),
        mix(a.a, b.a, t),
    )
}

