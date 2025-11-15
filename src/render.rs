use nalgebra::{Matrix4, Vector3 as Vec3, Vector4};
use std::f32::consts::PI;
use crate::vector::Vector3;
use crate::shaders::{ShaderColor, ShaderUniforms};
use crate::framebuffer::{Framebuffer, rgb_to_u32};
use crate::matrix::multiply_matrix_vector4;
use crate::planet::Planet;

pub const WIDTH: usize = 800;
pub const HEIGHT: usize = 600;

pub fn transform_vertex(v: &Vector3, matrix: &Matrix4<f32>) -> Vec3<f32> {
    let v4 = Vector4::new(v.x, v.y, v.z, 1.0);
    let transformed = multiply_matrix_vector4(matrix, &v4);
    Vec3::new(transformed.x, transformed.y, transformed.z)
}

pub fn project_vertex(v: &Vec3<f32>, projection: &Matrix4<f32>) -> Option<(i32, i32, f32)> {
    let v4 = Vector4::new(v.x, v.y, v.z, 1.0);
    let projected = multiply_matrix_vector4(projection, &v4);
    
    if projected.w <= 0.0 {
        return None;
    }
    
    let x = projected.x / projected.w;
    let y = projected.y / projected.w;
    let z = projected.z / projected.w;
    
    let screen_x = ((x + 1.0) * 0.5 * WIDTH as f32) as i32;
    let screen_y = ((1.0 - y) * 0.5 * HEIGHT as f32) as i32;
    
    Some((screen_x, screen_y, z))
}

fn edge_function(a: (f32, f32), b: (f32, f32), c: (f32, f32)) -> f32 {
    (c.0 - a.0) * (b.1 - a.1) - (c.1 - a.1) * (b.0 - a.0)
}

pub fn draw_triangle(
    framebuffer: &mut Framebuffer,
    v0: (i32, i32, f32), v1: (i32, i32, f32), v2: (i32, i32, f32),
    c0: ShaderColor, c1: ShaderColor, c2: ShaderColor,
) {
    let min_x = v0.0.min(v1.0).min(v2.0).max(0);
    let max_x = v0.0.max(v1.0).max(v2.0).min(WIDTH as i32 - 1);
    let min_y = v0.1.min(v1.1).min(v2.1).max(0);
    let max_y = v0.1.max(v1.1).max(v2.1).min(HEIGHT as i32 - 1);

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let p = (x as f32 + 0.5, y as f32 + 0.5);
            
            let w0 = edge_function((v1.0 as f32, v1.1 as f32), (v2.0 as f32, v2.1 as f32), p);
            let w1 = edge_function((v2.0 as f32, v2.1 as f32), (v0.0 as f32, v0.1 as f32), p);
            let w2 = edge_function((v0.0 as f32, v0.1 as f32), (v1.0 as f32, v1.1 as f32), p);
            
            if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
                let area = edge_function((v0.0 as f32, v0.1 as f32), (v1.0 as f32, v1.1 as f32), (v2.0 as f32, v2.1 as f32));
                
                if area.abs() < 0.001 {
                    continue;
                }
                
                let w0 = w0 / area;
                let w1 = w1 / area;
                let w2 = w2 / area;
                
                let depth = v0.2 * w0 + v1.2 * w1 + v2.2 * w2;
                
                let idx = y as usize * WIDTH + x as usize;
                if depth < framebuffer.depth_buffer[idx] {
                    framebuffer.depth_buffer[idx] = depth;
                    
                    let r = (c0.r * w0 + c1.r * w1 + c2.r * w2).clamp(0.0, 1.0);
                    let g = (c0.g * w0 + c1.g * w1 + c2.g * w2).clamp(0.0, 1.0);
                    let b = (c0.b * w0 + c1.b * w1 + c2.b * w2).clamp(0.0, 1.0);
                    
                    framebuffer.buffer[idx] = rgb_to_u32(
                        (r * 255.0) as u8,
                        (g * 255.0) as u8,
                        (b * 255.0) as u8,
                    );
                }
            }
        }
    }
}

fn draw_line_3d(
    framebuffer: &mut Framebuffer,
    start: Vec3<f32>,
    end: Vec3<f32>,
    view_proj: &Matrix4<f32>,
    color: u32,
) {
    if let (Some(p0), Some(p1)) = (
        project_vertex(&start, view_proj),
        project_vertex(&end, view_proj),
    ) {
        let mut x0 = p0.0;
        let mut y0 = p0.1;
        let x1 = p1.0;
        let y1 = p1.1;
        
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;
        
        loop {
            if x0 >= 0 && x0 < WIDTH as i32 && y0 >= 0 && y0 < HEIGHT as i32 {
                let idx = y0 as usize * WIDTH + x0 as usize;
                
                let t = if dx > -dy {
                    (x0 - p0.0) as f32 / (x1 - p0.0) as f32
                } else {
                    (y0 - p0.1) as f32 / (y1 - p0.1) as f32
                };
                let depth = p0.2 + (p1.2 - p0.2) * t.clamp(0.0, 1.0);
                
                if depth < framebuffer.depth_buffer[idx] {
                    framebuffer.buffer[idx] = color;
                }
            }
            
            if x0 == x1 && y0 == y1 {
                break;
            }
            
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }
    }
}

pub fn render_ecliptic_plane(
    framebuffer: &mut Framebuffer,
    view_proj: &Matrix4<f32>,
) {
    let orbit_color = rgb_to_u32(100, 140, 200);  
    let orbit_radii = vec![6.0, 9.0, 12.0, 15.0, 18.0]; 
    
    for &radius in &orbit_radii {
        let segments = 64;
        for i in 0..segments {
            let angle1 = (i as f32 / segments as f32) * 2.0 * PI;
            let angle2 = ((i + 1) as f32 / segments as f32) * 2.0 * PI;
            
            let x1 = angle1.cos() * radius;
            let z1 = angle1.sin() * radius;
            let x2 = angle2.cos() * radius;
            let z2 = angle2.sin() * radius;
            
            draw_line_3d(
                framebuffer,
                Vec3::new(x1, 0.0, z1),
                Vec3::new(x2, 0.0, z2),
                view_proj,
                orbit_color,
            );
        }
    }
}

pub fn render_planet(
    framebuffer: &mut Framebuffer,
    planet: &Planet,
    vertices: &[Vector3],
    normals: &[Vector3],
    uvs: &[(f32, f32)],
    indices: &[(usize, usize, usize)],
    view_proj: &Matrix4<f32>,
    uniforms: &ShaderUniforms,
) {
    let model = planet.get_model_matrix();
    let mvp = view_proj * model;

    for &(i0, i1, i2) in indices {
        let v0 = &vertices[i0];
        let v1 = &vertices[i1];
        let v2 = &vertices[i2];
        
        let n0 = &normals[i0];
        let n1 = &normals[i1];
        let n2 = &normals[i2];
        
        let uv0 = uvs[i0];
        let uv1 = uvs[i1];
        let uv2 = uvs[i2];

        let (pos0, norm0) = planet.shader.vertex_shader(*v0, *n0, uv0, uniforms);
        let (pos1, norm1) = planet.shader.vertex_shader(*v1, *n1, uv1, uniforms);
        let (pos2, norm2) = planet.shader.vertex_shader(*v2, *n2, uv2, uniforms);

        let world_v0 = transform_vertex(&pos0, &model);
        let world_v1 = transform_vertex(&pos1, &model);
        let world_v2 = transform_vertex(&pos2, &model);

        if let (Some(p0), Some(p1), Some(p2)) = (
            project_vertex(&world_v0, &mvp),
            project_vertex(&world_v1, &mvp),
            project_vertex(&world_v2, &mvp),
        ) {
            let c0 = planet.shader.fragment_shader(pos0, norm0, uv0, uniforms);
            let c1 = planet.shader.fragment_shader(pos1, norm1, uv1, uniforms);
            let c2 = planet.shader.fragment_shader(pos2, norm2, uv2, uniforms);

            draw_triangle(framebuffer, p0, p1, p2, c0, c1, c2);
        }
    }
}
