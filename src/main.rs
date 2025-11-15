mod vector;
mod shaders;
mod planets;
mod obj_loader;
mod framebuffer;
mod skybox;
mod camera;
mod matrix;
mod planet;
mod render;

use minifb::{Key, Window, WindowOptions};
use std::f32::consts::PI;
use vector::Vector3;
use shaders::ShaderUniforms;
use planets::*;
use obj_loader::ObjModel;
use framebuffer::Framebuffer;
use skybox::render_skybox;
use camera::Camera;
use matrix::create_projection_matrix;
use planet::Planet;
use render::{WIDTH, HEIGHT, render_planet, render_ecliptic_plane, project_vertex};

const FOV: f32 = PI / 3.0;
const NEAR: f32 = 0.1;
const FAR: f32 = 100.0;

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
        FOV,                     
        WIDTH as f32 / HEIGHT as f32,  
        NEAR,                     
        FAR,                     
    );

    // Crear el sol
    let sun = Planet::new(
        Box::new(SunShader),
        0.0,   
        2.3,   
        0.1,
        0.0,
        0.0,  
    );

    //Crear planetas
    let mut planets = vec![
        Planet::new(
            Box::new(RockyPlanetShader),
            2.7,   // Radio orbital
            1.2,   // Tamaño del planeta
            0.2,   
            0.3,   // Velocidad orbital
            0.0,   // Ángulo inicial
        ),
        Planet::new(
            Box::new(GasPlanetShader),
            5.3,   
            0.7,   
            0.15,
            0.2,   
            std::f32::consts::PI / 2.5,  
        ),
        Planet::new(
            Box::new(CrystalPlanetShader),
            6.5,  
            0.85,  
            0.2,
            0.15,  
            std::f32::consts::PI,  
        ),
        Planet::new(
            Box::new(NebulaPlanetShader),
            9.4,  
            0.6,  
            0.25,
            0.12,  
            4.0 * std::f32::consts::PI / 3.0,  
        ),
        Planet::new(
            Box::new(MetallicPlanetShader),
            8.6,  
            1.1,   
            0.12,
            0.1,   
            3.0 * std::f32::consts::PI / 2.0, 
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
    let mut frame_count = 0;
    let mut fps_timer = std::time::Instant::now();

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
        render_ecliptic_plane(&mut framebuffer, &view_proj);
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

        // Calcular FPS
        frame_count += 1;
        let elapsed = fps_timer.elapsed().as_secs_f32();
        if elapsed >= 1.0 {
            let current_fps = frame_count as f32 / elapsed;
            frame_count = 0;
            fps_timer = std::time::Instant::now();
            
            window.set_title(&format!("Space Travel | FPS: {:.1}", current_fps));
        }
    }
}
