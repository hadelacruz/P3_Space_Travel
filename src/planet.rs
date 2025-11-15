use nalgebra::{Matrix4, Vector3 as Vec3};
use crate::shaders::PlanetShader;
use crate::matrix::create_model_matrix;

pub struct Planet {
    pub shader: Box<dyn PlanetShader>,
    pub position: Vec3<f32>,
    pub scale: f32,
    pub _rotation: f32,
    pub _rotation_speed: f32,
    pub orbit_radius: f32,
    pub orbit_speed: f32,
    pub orbit_angle: f32,
}

impl Planet {
    pub fn new(
        shader: Box<dyn PlanetShader>,
        orbit_radius: f32,
        scale: f32,
        rotation_speed: f32,
        orbit_speed: f32,
        initial_angle: f32,
    ) -> Self {
        let initial_x = initial_angle.cos() * orbit_radius;
        let initial_z = initial_angle.sin() * orbit_radius;
        
        Planet {
            shader,
            position: Vec3::new(initial_x, 0.0, initial_z),
            scale,
            _rotation: 0.0,
            _rotation_speed: rotation_speed,
            orbit_radius,
            orbit_speed,
            orbit_angle: initial_angle,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.orbit_angle += self.orbit_speed * dt;
        
        self.position.x = self.orbit_angle.cos() * self.orbit_radius;
        self.position.z = self.orbit_angle.sin() * self.orbit_radius;
    }

    pub fn get_model_matrix(&self) -> Matrix4<f32> {
        create_model_matrix(
            self.position,
            self.scale,
            Vec3::new(0.0, 0.0, 0.0)
        )
    }
}
