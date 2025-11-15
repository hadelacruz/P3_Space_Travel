use crate::vector::Vector3;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct ObjModel {
    pub vertices: Vec<Vector3>,
    pub normals: Vec<Vector3>,
    pub uvs: Vec<(f32, f32)>,
    pub indices: Vec<(usize, usize, usize)>,
}

impl ObjModel {
    pub fn load(filename: &str) -> Result<Self, String> {
        let file = File::open(filename)
            .map_err(|e| format!("No se pudo abrir el archivo {}: {}", filename, e))?;
        
        let reader = BufReader::new(file);
        
        let mut indices: Vec<(usize, usize, usize)> = Vec::new();
        
        let mut temp_vertices: Vec<Vector3> = Vec::new();
        let mut temp_normals: Vec<Vector3> = Vec::new();
        let mut temp_uvs: Vec<(f32, f32)> = Vec::new();
        
        for line in reader.lines() {
            let line = line.map_err(|e| format!("Error leyendo línea: {}", e))?;
            let parts: Vec<&str> = line.split_whitespace().collect();
            
            if parts.is_empty() {
                continue;
            }
            
            match parts[0] {
                "v" if parts.len() >= 4 => {
                    // Vértice
                    let x: f32 = parts[1].parse().unwrap_or(0.0);
                    let y: f32 = parts[2].parse().unwrap_or(0.0);
                    let z: f32 = parts[3].parse().unwrap_or(0.0);
                    temp_vertices.push(Vector3::new(x, y, z));
                }
                "vn" if parts.len() >= 4 => {
                    // Normal
                    let x: f32 = parts[1].parse().unwrap_or(0.0);
                    let y: f32 = parts[2].parse().unwrap_or(0.0);
                    let z: f32 = parts[3].parse().unwrap_or(0.0);
                    temp_normals.push(Vector3::new(x, y, z));
                }
                "vt" if parts.len() >= 3 => {
                    // Coordenada de textura
                    let u: f32 = parts[1].parse().unwrap_or(0.0);
                    let v: f32 = parts[2].parse().unwrap_or(0.0);
                    temp_uvs.push((u, v));
                }
                "f" if parts.len() >= 4 => {
                    // Cara (triángulo)
                    let mut face_vertices = Vec::new();
                    
                    for i in 1..parts.len() {
                        let face_data: Vec<&str> = parts[i].split('/').collect();
                        
                        if !face_data.is_empty() {
                            if let Ok(v_idx) = face_data[0].parse::<usize>() {
                                face_vertices.push(v_idx - 1); // OBJ usa índices base 1
                            }
                        }
                    }
                    
                    // Triangular caras con más de 3 vértices
                    if face_vertices.len() >= 3 {
                        for i in 1..face_vertices.len() - 1 {
                            indices.push((
                                face_vertices[0],
                                face_vertices[i],
                                face_vertices[i + 1]
                            ));
                        }
                    }
                }
                _ => {}
            }
        }
        
        if temp_normals.is_empty() {
            temp_normals = vec![Vector3::new(0.0, 0.0, 0.0); temp_vertices.len()];
            
            for &(i0, i1, i2) in &indices {
                if i0 < temp_vertices.len() && i1 < temp_vertices.len() && i2 < temp_vertices.len() {
                    let v0 = temp_vertices[i0];
                    let v1 = temp_vertices[i1];
                    let v2 = temp_vertices[i2];
                    
                    let edge1 = Vector3::new(v1.x - v0.x, v1.y - v0.y, v1.z - v0.z);
                    let edge2 = Vector3::new(v2.x - v0.x, v2.y - v0.y, v2.z - v0.z);
                    let normal = edge1.cross(&edge2).normalize();
                    
                    temp_normals[i0] = Vector3::new(
                        temp_normals[i0].x + normal.x,
                        temp_normals[i0].y + normal.y,
                        temp_normals[i0].z + normal.z,
                    );
                    temp_normals[i1] = Vector3::new(
                        temp_normals[i1].x + normal.x,
                        temp_normals[i1].y + normal.y,
                        temp_normals[i1].z + normal.z,
                    );
                    temp_normals[i2] = Vector3::new(
                        temp_normals[i2].x + normal.x,
                        temp_normals[i2].y + normal.y,
                        temp_normals[i2].z + normal.z,
                    );
                }
            }
            
            for normal in &mut temp_normals {
                *normal = normal.normalize();
            }
        }
        
        if temp_uvs.is_empty() {
            for vertex in &temp_vertices {
                let u = 0.5 + vertex.x.atan2(vertex.z) / (2.0 * std::f32::consts::PI);
                let v = 0.5 - vertex.y.asin() / std::f32::consts::PI;
                temp_uvs.push((u, v));
            }
        }
        
        Ok(ObjModel {
            vertices: temp_vertices,
            normals: temp_normals,
            uvs: temp_uvs,
            indices,
        })
    }
}
