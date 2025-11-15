use nalgebra::Vector3 as Vec3;
use std::f32::consts::PI;
use crate::framebuffer::{Framebuffer, rgb_to_u32};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

pub fn generate_star_positions(count: usize, seed: f32) -> Vec<Vec3<f32>> {
    let mut stars = Vec::new();
    let distance = 95.0; 
    
    for i in 0..count {
        let t = i as f32 * seed;
        
        let hash_x = ((t * 12.9898 + 78.233).sin() * 43758.5453).fract();
        let hash_y = ((t * 93.9898 + 67.345).sin() * 28371.4573).fract();
        
        // Convertir a coordenadas esféricas para distribución uniforme
        let theta = hash_x * 2.0 * PI; // Ángulo horizontal (0 a 2π)
        let phi = (hash_y * 2.0 - 1.0).acos(); // Ángulo vertical (0 a π) 
        
        // Convertir a coordenadas cartesianas
        let x = distance * phi.sin() * theta.cos();
        let y = distance * phi.sin() * theta.sin();
        let z = distance * phi.cos();
        
        stars.push(Vec3::new(x, y, z));
    }
    
    stars
}

/// Renderiza el skybox con estrellas en el fondo
pub fn render_skybox(
    framebuffer: &mut Framebuffer,
    view_proj: &nalgebra::Matrix4<f32>,
    time: f32,
    project_fn: impl Fn(&Vec3<f32>, &nalgebra::Matrix4<f32>) -> Option<(i32, i32, f32)>,
) {
    let stars = generate_star_positions(800, 2.71828);
    
    for (i, star_pos) in stars.iter().enumerate() {
        if let Some((sx, sy, sz)) = project_fn(star_pos, view_proj) {
            if sx >= 0 && sx < WIDTH as i32 && sy >= 0 && sy < HEIGHT as i32 {
                let idx = sy as usize * WIDTH + sx as usize;
                
                if sz < framebuffer.depth_buffer[idx] {
                    let twinkle = ((time * 2.0 + i as f32 * 0.1).sin() * 0.5 + 0.5) * 0.3 + 0.7;
                    
                    let brightness_hash = (i as f32 * 7.123).sin() * 0.5 + 0.5;
                    let brightness = (brightness_hash * 150.0 + 105.0) * twinkle;
                    
                    let is_warm = ((i as f32 * 3.456).sin() * 0.5 + 0.5) > 0.7;
                    
                    let (r, g, b) = if is_warm {
                        (
                            brightness as u8,
                            (brightness * 0.9) as u8,
                            (brightness * 0.7) as u8,
                        )
                    } else {
                        (
                            (brightness * 0.9) as u8,
                            (brightness * 0.95) as u8,
                            brightness as u8,
                        )
                    };
                    
                    framebuffer.depth_buffer[idx] = sz;
                    framebuffer.buffer[idx] = rgb_to_u32(r, g, b);
                    
                    if brightness > 200.0 {
                        for dy in -1..=1 {
                            for dx in -1..=1 {
                                if dx == 0 && dy == 0 { continue; }
                                let nx = sx + dx;
                                let ny = sy + dy;
                                if nx >= 0 && nx < WIDTH as i32 && ny >= 0 && ny < HEIGHT as i32 {
                                    let nidx = ny as usize * WIDTH + nx as usize;
                                    if sz < framebuffer.depth_buffer[nidx] {
                                        let halo_brightness = brightness * 0.3;
                                        let (hr, hg, hb) = if is_warm {
                                            (
                                                halo_brightness as u8,
                                                (halo_brightness * 0.9) as u8,
                                                (halo_brightness * 0.7) as u8,
                                            )
                                        } else {
                                            (
                                                (halo_brightness * 0.9) as u8,
                                                (halo_brightness * 0.95) as u8,
                                                halo_brightness as u8,
                                            )
                                        };
                                        framebuffer.buffer[nidx] = rgb_to_u32(hr, hg, hb);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
