// ============================================================================
// PLANETA 1: PLANETA ROCOSO CON DEFORMACIÓN PROCEDURAL (VERTEX SHADER)
// Características: Deformación geométrica procedural, terreno con relieve, colores grises
// ============================================================================

use crate::vector::Vector3;
use crate::shaders::{ShaderColor, ShaderUniforms, PlanetShader, fbm3d, voronoi_noise, fbm, ridge_noise, smoothstep, mix_color};

pub struct RockyPlanetShader;

impl PlanetShader for RockyPlanetShader {
    fn vertex_shader(&self, position: Vector3, normal: Vector3, _uv: (f32, f32), uniforms: &ShaderUniforms) -> (Vector3, Vector3) {
        // === DEFORMACIÓN PROCEDURAL DEL TERRENO ===
        
        // Capa 1: Montañas grandes (escala global)
        let mountain_noise = fbm3d(
            position.x * 2.0,
            position.y * 2.0,
            position.z * 2.0,
            4
        ) * 0.15;
        
        // Capa 2: Colinas medianas
        let hill_noise = fbm3d(
            position.x * 5.0,
            position.y * 5.0,
            position.z * 5.0,
            3
        ) * 0.08;
        
        // Capa 3: Detalles finos (rocas pequeñas)
        let detail_noise = fbm3d(
            position.x * 15.0,
            position.y * 15.0,
            position.z * 15.0,
            2
        ) * 0.03;
        
        // Capa 4: Cráteres procedurales
        let crater_x = position.x * 8.0;
        let crater_y = position.y * 8.0;
        let crater_pattern = voronoi_noise(crater_x, crater_y);
        let crater_depth = if crater_pattern < 0.2 {
            -0.05 * (1.0 - crater_pattern / 0.2)
        } else {
            0.0
        };
        
        // Capa 5: Animación sutil (pulso tectónico)
        let tectonic_pulse = (uniforms.time * 0.5).sin() * 0.01;
        let pulse_noise = fbm3d(
            position.x * 3.0 + uniforms.time * 0.1,
            position.y * 3.0,
            position.z * 3.0,
            2
        ) * tectonic_pulse;
        
        // Combinar todas las deformaciones
        let total_displacement = mountain_noise + hill_noise + detail_noise + crater_depth + pulse_noise;
        
        // Aplicar deformación a lo largo de la normal
        let deformed_position = Vector3::new(
            position.x + normal.x * total_displacement,
            position.y + normal.y * total_displacement,
            position.z + normal.z * total_displacement,
        );
        
        // Recalcular normal aproximada basada en deformación
        let epsilon = 0.01;
        let neighbor_noise = fbm3d(
            (position.x + epsilon) * 2.0,
            (position.y + epsilon) * 2.0,
            (position.z + epsilon) * 2.0,
            4
        ) * 0.15;
        
        let tangent_displacement = neighbor_noise - mountain_noise;
        let normal_perturbation = Vector3::new(
            -tangent_displacement,
            -tangent_displacement,
            -tangent_displacement,
        ) * 0.3;
        
        let perturbed_normal = Vector3::new(
            normal.x + normal_perturbation.x,
            normal.y + normal_perturbation.y,
            normal.z + normal_perturbation.z,
        ).normalize();
        
        (deformed_position, perturbed_normal)
    }

    fn fragment_shader(&self, position: Vector3, normal: Vector3, uv: (f32, f32), uniforms: &ShaderUniforms) -> ShaderColor {
        // === PALETA DE COLORES GRISES (NO CAMBIA CON EL TIEMPO) ===
        let darkest_gray = ShaderColor::from_rgb(20, 20, 25);      // Gris casi negro
        let dark_gray = ShaderColor::from_rgb(50, 50, 55);         // Gris oscuro
        let medium_dark = ShaderColor::from_rgb(80, 80, 85);       // Gris medio-oscuro
        let medium_gray = ShaderColor::from_rgb(110, 110, 115);    // Gris medio
        let light_gray = ShaderColor::from_rgb(140, 140, 145);     // Gris claro
        let lighter_gray = ShaderColor::from_rgb(170, 170, 175);   // Gris más claro
        let lightest_gray = ShaderColor::from_rgb(200, 200, 205);  // Gris casi blanco
        
        // === CAPA 1: TEXTURA BASE (Variación de rocas) ===
        let rock_variation = fbm3d(
            position.x * 8.0,
            position.y * 8.0,
            position.z * 8.0,
            5
        );
        
        // === CAPA 2: DETALLES GEOLÓGICOS (Erosión y fracturas) ===
        let erosion = fbm(uv.0 * 20.0, uv.1 * 20.0, 4);
        let fractures = ridge_noise(uv.0 * 15.0, uv.1 * 15.0, 3);
        
        // === CAPA 3: CRÁTERES (Oscurecimiento) ===
        let crater_noise = voronoi_noise(uv.0 * 8.0, uv.1 * 8.0);
        let is_crater = crater_noise < 0.2;
        
        // === CAPA 4: VETAS MINERALES (Líneas más claras) ===
        let mineral_veins = ridge_noise(uv.0 * 25.0, uv.1 * 25.0, 2);
        let has_veins = mineral_veins > 0.75;
        
        // === SELECCIÓN DE COLOR BASE (Solo grises, NO cambia) ===
        let base_color = if rock_variation < 0.2 {
            mix_color(darkest_gray, dark_gray, smoothstep(0.0, 0.2, rock_variation))
        } else if rock_variation < 0.4 {
            mix_color(dark_gray, medium_dark, smoothstep(0.2, 0.4, rock_variation))
        } else if rock_variation < 0.6 {
            mix_color(medium_dark, medium_gray, smoothstep(0.4, 0.6, rock_variation))
        } else if rock_variation < 0.75 {
            mix_color(medium_gray, light_gray, smoothstep(0.6, 0.75, rock_variation))
        } else if rock_variation < 0.85 {
            mix_color(light_gray, lighter_gray, smoothstep(0.75, 0.85, rock_variation))
        } else {
            mix_color(lighter_gray, lightest_gray, smoothstep(0.85, 1.0, rock_variation))
        };
        
        let mut final_base = base_color;
        
        // === APLICAR CRÁTERES (Oscurecer) ===
        if is_crater {
            final_base = mix_color(final_base, darkest_gray, 0.6);
        }
        
        // === APLICAR EROSIÓN (Variación sutil) ===
        if erosion > 0.6 {
            let erosion_factor = smoothstep(0.6, 0.8, erosion);
            final_base = mix_color(final_base, dark_gray, erosion_factor * 0.3);
        }
        
        // === APLICAR FRACTURAS (Oscurecer líneas) ===
        if fractures > 0.7 {
            let fracture_intensity = smoothstep(0.7, 0.85, fractures);
            final_base = mix_color(final_base, darkest_gray, fracture_intensity * 0.5);
        }
        
        // === APLICAR VETAS MINERALES (Aclarar líneas) ===
        if has_veins {
            let vein_intensity = smoothstep(0.75, 0.9, mineral_veins);
            final_base = mix_color(final_base, lightest_gray, vein_intensity * 0.4);
        }
        
        // === ILUMINACIÓN (Sin colores, solo intensidad) ===
        let light_dir = uniforms.light_direction.normalize();
        let view_dir = (uniforms.camera_position - position).normalize();
        
        // Difusa básica
        let diffuse = normal.dot(&light_dir).max(0.0);
        
        // Especular suave para rocas
        let reflect_dir = normal * (2.0 * normal.dot(&light_dir)) - light_dir;
        let specular = view_dir.dot(&reflect_dir).max(0.0).powf(8.0) * 0.2;
        
        // Oclusión ambiental basada en curvatura
        let ambient_occlusion = (1.0 - erosion * 0.3).max(0.3);
        
        let ambient = 0.2;
        let lighting_intensity = (ambient * ambient_occlusion + diffuse * 0.7 + specular).min(1.2);
        
        // Color final (SOLO GRISES, sin cambios de color)
        let final_color = ShaderColor::new(
            final_base.r * lighting_intensity,
            final_base.g * lighting_intensity,
            final_base.b * lighting_intensity,
            1.0,
        );
        
        ShaderColor::new(
            final_color.r.clamp(0.0, 1.0),
            final_color.g.clamp(0.0, 1.0),
            final_color.b.clamp(0.0, 1.0),
            1.0,
        )
    }
}
