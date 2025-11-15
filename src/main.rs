mod vector;
mod shaders;
mod planets;
mod obj_loader;
mod framebuffer;
mod skybox;
mod camera;
mod matrix;

use minifb::{Key, Window, WindowOptions};
use nalgebra::{Matrix4, Vector3 as Vec3, Vector4};
use std::f32::consts::PI;
use vector::Vector3;
use shaders::{ShaderColor, ShaderUniforms, PlanetShader};
use planets::*;
use obj_loader::ObjModel;
use framebuffer::{Framebuffer, rgb_to_u32};
use skybox::render_skybox;
use camera::Camera;
use matrix::{create_model_matrix, multiply_matrix_vector4, create_projection_matrix};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const FOV: f32 = PI / 3.0;
const NEAR: f32 = 0.1;
const FAR: f32 = 100.0;

struct Planet {
    shader: Box<dyn PlanetShader>,
    position: Vec3<f32>,
    scale: f32,
    rotation: f32,
    rotation_speed: f32,
    orbit_radius: f32,
    orbit_speed: f32,
    orbit_angle: f32,
}

impl Planet {
    fn new(
        shader: Box<dyn PlanetShader>,
        orbit_radius: f32,
        scale: f32,
        rotation_speed: f32,
        orbit_speed: f32,
        initial_angle: f32,
    ) -> Self {
        // Calcular posición inicial basada en el ángulo
        let initial_x = initial_angle.cos() * orbit_radius;
        let initial_z = initial_angle.sin() * orbit_radius;
        
        Planet {
            shader,
            position: Vec3::new(initial_x, 0.0, initial_z),
            scale,
            rotation: 0.0,
            rotation_speed,
            orbit_radius,
            orbit_speed,
            orbit_angle: initial_angle,
        }
    }

    fn update(&mut self, dt: f32) {
        self.orbit_angle += self.orbit_speed * dt;
        
        self.position.x = self.orbit_angle.cos() * self.orbit_radius;
        self.position.z = self.orbit_angle.sin() * self.orbit_radius;
    }

    fn get_model_matrix(&self) -> Matrix4<f32> {
        // Usar la función manual de creación de matriz de modelo
        create_model_matrix(
            self.position,
            self.scale,
            Vec3::new(0.0, 0.0, 0.0) // Sin rotación por ahora
        )
    }
}

fn transform_vertex(v: &Vector3, matrix: &Matrix4<f32>) -> Vec3<f32> {
    let v4 = Vector4::new(v.x, v.y, v.z, 1.0);
    // Usar la función manual de multiplicación matriz-vector
    let transformed = multiply_matrix_vector4(matrix, &v4);
    Vec3::new(transformed.x, transformed.y, transformed.z)
}

fn project_vertex(v: &Vec3<f32>, projection: &Matrix4<f32>) -> Option<(i32, i32, f32)> {
    let v4 = Vector4::new(v.x, v.y, v.z, 1.0);
    // Usar la función manual de multiplicación matriz-vector
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

fn draw_triangle(
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

fn edge_function(a: (f32, f32), b: (f32, f32), c: (f32, f32)) -> f32 {
    (c.0 - a.0) * (b.1 - a.1) - (c.1 - a.1) * (b.0 - a.0)
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
                
                // Interpolar profundidad
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

fn render_ecliptic_plane(
    framebuffer: &mut Framebuffer,
    view_proj: &Matrix4<f32>,
    _max_radius: f32,
) {
    // Dibujar círculos de órbita para cada planeta 
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



fn render_planet(
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

fn main() {
    let mut window = Window::new(
        "Sistema Solar - Space Travel",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.set_target_fps(60);

    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);

    let mut camera = Camera::new(40.0);
    // Usar la función manual de creación de matriz de proyección
    let projection = create_projection_matrix(
        FOV,                      // fov_y en radianes
        WIDTH as f32 / HEIGHT as f32,  // aspect ratio
        NEAR,                     // near plane
        FAR,                      // far plane
    );

    // Crear el sol
    let sun = Planet::new(
        Box::new(SunShader),
        0.0,   
        2.0,   
        0.1,
        0.0,
        0.0,  
    );

    //Crear planetas
    let mut planets = vec![
        Planet::new(
            Box::new(RockyPlanetShader),
            2.7,   // Radio orbital - órbitas más juntas
            1.2,   // Más pequeño para claridad
            0.2,
            0.3,   // Velocidad orbital
            0.0,   
        ),
        Planet::new(
            Box::new(GasPlanetShader),
            5.3,   // Radio orbital - órbitas más juntas
            0.7,   // Reducido para que no tape otras órbitas
            0.15,
            0.2,   // Velocidad orbital
            std::f32::consts::PI / 2.5,  // Comienza en ~72°
        ),
        Planet::new(
            Box::new(CrystalPlanetShader),
            6.5,  // Radio orbital - órbitas más juntas
            0.85,  // Más pequeño para claridad
            0.2,
            0.15,  // Velocidad orbital
            std::f32::consts::PI,  
        ),
        Planet::new(
            Box::new(NebulaPlanetShader),
            9.3,  // Radio orbital - órbitas más juntas
            0.6,  // Más pequeño para claridad
            0.25,
            0.12,  // Velocidad orbital
            4.0 * std::f32::consts::PI / 3.0,  // Comienza en ~240°
        ),
        Planet::new(
            Box::new(MetallicPlanetShader),
            8.6,  // Radio orbital - órbitas más juntas
            1.1,   // Reducido para claridad
            0.12,
            0.1,   // Velocidad orbital
            3.0 * std::f32::consts::PI / 2.0,  // Comienza en 270°
        ),
    ];

    let sphere_model = ObjModel::load("sphere.obj")
        .expect("No se pudo cargar el archivo sphere.obj");
    
    let vertices = sphere_model.vertices;
    let normals = sphere_model.normals;
    let uvs = sphere_model.uvs;
    let indices = sphere_model.indices;

    println!("Modelo cargado: {} vértices, {} triángulos", vertices.len(), indices.len());

    let start_time = std::time::Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let dt = 0.016; 
        let time = start_time.elapsed().as_secs_f32();

        if window.is_key_down(Key::Left) {
            camera.rotate(-0.05);
        }
        if window.is_key_down(Key::Right) {
            camera.rotate(0.05);
        }
        if window.is_key_down(Key::Up) {
            camera.zoom(-0.5);
        }
        if window.is_key_down(Key::Down) {
            camera.zoom(0.5);
        }
        if window.is_key_down(Key::W) {
            camera.change_height(0.3);
        }
        if window.is_key_down(Key::S) {
            camera.change_height(-0.3);
        }

        for planet in &mut planets {
            planet.update(dt);
        }

        framebuffer.clear();

        let view = camera.get_view_matrix();
        let view_proj = projection * view;

        let uniforms = ShaderUniforms {
            time,
            light_direction: Vector3::new(0.0, 0.0, 1.0).normalize(),
            camera_position: Vector3::new(camera.position.x, camera.position.y, camera.position.z),
        };

        render_skybox(&mut framebuffer, &view_proj, time, project_vertex);
        render_ecliptic_plane(&mut framebuffer, &view_proj, 50.0);
        render_planet(
            &mut framebuffer,
            &sun,
            &vertices,
            &normals,
            &uvs,
            &indices,
            &view_proj,
            &uniforms,
        );
        for planet in &planets {
            render_planet(
                &mut framebuffer,
                planet,
                &vertices,
                &normals,
                &uvs,
                &indices,
                &view_proj,
                &uniforms,
            );
        }

        window
            .update_with_buffer(framebuffer.get_buffer(), WIDTH, HEIGHT)
            .unwrap();
    }
}
