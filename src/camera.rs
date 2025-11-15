use nalgebra::{Vector3 as Vec3};
use crate::matrix::create_view_matrix;
use nalgebra::Matrix4;

pub struct Camera {
    pub position: Vec3<f32>,
    pub target: Vec3<f32>,
    pub up: Vec3<f32>,
    angle: f32,
    distance: f32,
    height: f32,
}

impl Camera {
    pub fn new(distance: f32) -> Self {
        Camera {
            position: Vec3::new(0.0, 5.0, distance),
            target: Vec3::new(0.0, 0.0, 0.0),
            up: Vec3::new(0.0, 1.0, 0.0),
            angle: 0.0,
            distance,
            height: 5.0,
        }
    }

    fn update_position(&mut self) {
        self.position.x = self.angle.cos() * self.distance;
        self.position.z = self.angle.sin() * self.distance;
        self.position.y = self.height;
    }

    pub fn rotate(&mut self, delta_angle: f32) {
        self.angle += delta_angle;
        self.update_position();
    }

    pub fn zoom(&mut self, delta: f32) {
        self.distance = (self.distance + delta).max(10.0).min(100.0);
        self.update_position();
    }

    pub fn change_height(&mut self, delta: f32) {
        self.height = (self.height + delta).max(2.0).min(20.0);
        self.update_position();
    }

    pub fn get_view_matrix(&self) -> Matrix4<f32> {
        // Usar la función manual de creación de matriz de vista
        create_view_matrix(self.position, self.target, self.up)
    }
}
