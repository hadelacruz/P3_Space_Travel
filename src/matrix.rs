use nalgebra::{Matrix4, Vector3, Vector4};

pub fn multiply_matrix_vector4(matrix: &Matrix4<f32>, vector: &Vector4<f32>) -> Vector4<f32> {
    Vector4::new(
        matrix.m11 * vector.x + matrix.m12 * vector.y + matrix.m13 * vector.z + matrix.m14 * vector.w,
        matrix.m21 * vector.x + matrix.m22 * vector.y + matrix.m23 * vector.z + matrix.m24 * vector.w,
        matrix.m31 * vector.x + matrix.m32 * vector.y + matrix.m33 * vector.z + matrix.m34 * vector.w,
        matrix.m41 * vector.x + matrix.m42 * vector.y + matrix.m43 * vector.z + matrix.m44 * vector.w,
    )
}

/// Crea una matriz 4x4 desde 16 valores en orden row-major
pub fn create_matrix4(
    r0c0: f32, r0c1: f32, r0c2: f32, r0c3: f32,
    r1c0: f32, r1c1: f32, r1c2: f32, r1c3: f32,
    r2c0: f32, r2c1: f32, r2c2: f32, r2c3: f32,
    r3c0: f32, r3c1: f32, r3c2: f32, r3c3: f32,
) -> Matrix4<f32> {
    Matrix4::new(
        r0c0, r0c1, r0c2, r0c3,
        r1c0, r1c1, r1c2, r1c3,
        r2c0, r2c1, r2c2, r2c3,
        r3c0, r3c1, r3c2, r3c3,
    )
}

pub fn create_model_matrix(translation: Vector3<f32>, scale: f32, rotation: Vector3<f32>) -> Matrix4<f32> {
    let (sin_x, cos_x) = rotation.x.sin_cos();
    let (sin_y, cos_y) = rotation.y.sin_cos();
    let (sin_z, cos_z) = rotation.z.sin_cos();

    // Rotación alrededor del eje X
    let rotation_matrix_x = create_matrix4(
        1.0, 0.0,    0.0,    0.0,
        0.0, cos_x,  -sin_x, 0.0,
        0.0, sin_x,  cos_x,  0.0,
        0.0, 0.0,    0.0,    1.0
    );

    // Rotación alrededor del eje Y
    let rotation_matrix_y = create_matrix4(
        cos_y,  0.0, sin_y, 0.0,
        0.0,    1.0, 0.0,   0.0,
        -sin_y, 0.0, cos_y, 0.0,
        0.0,    0.0, 0.0,   1.0
    );

    // Rotación alrededor del eje Z
    let rotation_matrix_z = create_matrix4(
        cos_z, -sin_z, 0.0, 0.0,
        sin_z, cos_z,  0.0, 0.0,
        0.0,   0.0,    1.0, 0.0,
        0.0,   0.0,    0.0, 1.0
    );

    let rotation_matrix = rotation_matrix_z * rotation_matrix_y * rotation_matrix_x;

    // Matriz de escala
    let scale_matrix = create_matrix4(
        scale, 0.0,   0.0,   0.0,
        0.0,   scale, 0.0,   0.0,
        0.0,   0.0,   scale, 0.0,
        0.0,   0.0,   0.0,   1.0
    );

    // Matriz de traslación
    let translation_matrix = create_matrix4(
        1.0, 0.0, 0.0, translation.x,
        0.0, 1.0, 0.0, translation.y,
        0.0, 0.0, 1.0, translation.z,
        0.0, 0.0, 0.0, 1.0
    );

    translation_matrix * rotation_matrix * scale_matrix
}

pub fn create_view_matrix(eye: Vector3<f32>, target: Vector3<f32>, up: Vector3<f32>) -> Matrix4<f32> {
    let mut forward = Vector3::new(
        target.x - eye.x,
        target.y - eye.y,
        target.z - eye.z,
    );
    // Normalizar forward
    let forward_length = (forward.x * forward.x + forward.y * forward.y + forward.z * forward.z).sqrt();
    forward.x /= forward_length;
    forward.y /= forward_length;
    forward.z /= forward_length;

    // Calcular vector right
    let mut right = Vector3::new(
        forward.y * up.z - forward.z * up.y,
        forward.z * up.x - forward.x * up.z,
        forward.x * up.y - forward.y * up.x,
    );
    // Normalizar right
    let right_length = (right.x * right.x + right.y * right.y + right.z * right.z).sqrt();
    right.x /= right_length;
    right.y /= right_length;
    right.z /= right_length;

    // Calcular vector up real 
    let actual_up = Vector3::new(
        right.y * forward.z - right.z * forward.y,
        right.z * forward.x - right.x * forward.z,
        right.x * forward.y - right.y * forward.x,
    );

    // Crear la matriz de vista
    create_matrix4(
        right.x, right.y, right.z, -(right.x * eye.x + right.y * eye.y + right.z * eye.z),
        actual_up.x, actual_up.y, actual_up.z, -(actual_up.x * eye.x + actual_up.y * eye.y + actual_up.z * eye.z),
        -forward.x, -forward.y, -forward.z, forward.x * eye.x + forward.y * eye.y + forward.z * eye.z,
        0.0, 0.0, 0.0, 1.0,
    )
}

// Crea una matriz de proyección en perspectiva
pub fn create_projection_matrix(fov_y: f32, aspect: f32, near: f32, far: f32) -> Matrix4<f32> {
    let tan_half_fov = (fov_y / 2.0).tan();

    create_matrix4(
        1.0 / (aspect * tan_half_fov), 0.0, 0.0, 0.0,
        0.0, 1.0 / tan_half_fov, 0.0, 0.0,
        0.0, 0.0, -(far + near) / (far - near), -(2.0 * far * near) / (far - near),
        0.0, 0.0, -1.0, 0.0,
    )
}
