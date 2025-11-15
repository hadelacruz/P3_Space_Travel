use crate::vector::Vector3;
use crate::shaders::{ShaderColor, ShaderUniforms, PlanetShader, fbm, fbm3d, voronoi_noise, smoothstep, mix_color};

pub struct MetallicPlanetShader;

impl PlanetShader for MetallicPlanetShader {
    fn vertex_shader(&self, position: Vector3, normal: Vector3, _uv: (f32, f32), uniforms: &ShaderUniforms) -> (Vector3, Vector3) {
        // === GENERAR PICOS/PÚAS PROCEDURALMENTE CON ANIMACIÓN ===
        
        // Ondas de animación que recorren el planeta
        let wave1 = (uniforms.time * 1.5 + position.x * 3.0).sin() * 0.5 + 0.5;
        let wave2 = (uniforms.time * 2.0 + position.y * 4.0).cos() * 0.5 + 0.5;
        let wave3 = (uniforms.time * 1.8 + position.z * 3.5).sin() * 0.5 + 0.5;
        let wave_combined = (wave1 + wave2 + wave3) / 3.0;
        
        // CAPA 1: Picos grandes principales con crecimiento animado
        let voronoi_scale = 15.0;
        let time_offset_large = uniforms.time * 0.3;
        let voronoi_pattern = voronoi_noise(
            position.x * voronoi_scale + position.z * voronoi_scale + time_offset_large,
            position.y * voronoi_scale + time_offset_large * 0.5
        );
        
        // Animación de crecimiento/retracción de picos grandes
        let growth_anim = (uniforms.time * 1.2 + voronoi_pattern * 10.0).sin() * 0.15 + 1.0;
        let spike_large = smoothstep(0.15, 0.05, voronoi_pattern).max(0.0) * 0.35 * growth_anim;
        
        // CAPA 2: Picos medianos con rotación
        let time_offset_medium = uniforms.time * 0.5;
        let rotation = uniforms.time * 0.4;
        let pos_rotated_x = position.x * rotation.cos() - position.z * rotation.sin();
        let _pos_rotated_z = position.x * rotation.sin() + position.z * rotation.cos();
        
        let voronoi_medium = voronoi_noise(
            pos_rotated_x * 25.0 + time_offset_medium,
            position.y * 25.0 + time_offset_medium * 0.7
        );
        
        let pulse_medium = (uniforms.time * 2.5 + voronoi_medium * 8.0).cos() * 0.12 + 1.0;
        let spike_medium = smoothstep(0.12, 0.04, voronoi_medium).max(0.0) * 0.25 * pulse_medium;
        
        // CAPA 3: Picos pequeños con vibración rápida
        let vibration = (uniforms.time * 4.0 + position.length() * 10.0).sin() * 0.08;
        let time_offset_small = uniforms.time * 0.8;
        
        let voronoi_small = voronoi_noise(
            position.x * 40.0 + time_offset_small + vibration,
            position.y * 40.0 + time_offset_small * 1.2
        );
        let spike_small = smoothstep(0.1, 0.03, voronoi_small).max(0.0) * 0.15 * wave_combined;
        
        // CAPA 4: Rugosidad base animada
        let roughness = fbm3d(
            position.x * 50.0 + uniforms.time * 0.2,
            position.y * 50.0 + uniforms.time * 0.15,
            position.z * 50.0 + uniforms.time * 0.25,
            4
        ) * 0.05;
        
        // CAPA 5: Pulsación metálica global
        let pulse = (uniforms.time * 2.0 + position.length() * 5.0).sin() * 0.02;
        
        // CAPA 6: Ondas de energía que recorren las espinas
        let energy_wave = ((uniforms.time * 3.0 + position.x * 5.0 + position.y * 5.0).sin() * 
                          (uniforms.time * 2.5 + position.z * 4.0).cos()) * 0.04;
        
        let total_displacement = spike_large + spike_medium + spike_small + roughness + pulse + energy_wave;        
        let displaced_position = position + normal * total_displacement;
        
        // Ajustar la normal con la animación de las espinas
        let spike_factor = spike_large * 2.0 + spike_medium * 1.5 + spike_small;
        let animated_normal_factor = spike_factor * (1.0 + wave_combined * 0.3);
        let adjusted_normal = (normal + normal * animated_normal_factor).normalize();
        
        (displaced_position, adjusted_normal)
    }

    fn fragment_shader(&self, position: Vector3, normal: Vector3, uv: (f32, f32), uniforms: &ShaderUniforms) -> ShaderColor {
        // === PALETA METÁLICA ===
        let dark_metal = ShaderColor::from_rgb(40, 45, 50);        
        let medium_metal = ShaderColor::from_rgb(80, 90, 100);     
        let light_metal = ShaderColor::from_rgb(140, 150, 160);    
        let bright_metal = ShaderColor::from_rgb(200, 210, 220);   
        let chrome = ShaderColor::from_rgb(240, 245, 250);        
        let rust_accent = ShaderColor::from_rgb(120, 80, 60);     
        
        // === TEXTURA METÁLICA PROCEDURAL ===
        
        let metal_pattern = voronoi_noise(
            position.x * 20.0 + position.z * 20.0,
            position.y * 20.0
        );
        
        let imperfections = fbm3d(
            position.x * 30.0,
            position.y * 30.0,
            position.z * 30.0,
            4
        );
        
        let scratches = fbm(
            uv.0 * 100.0,
            uv.1 * 100.0,
            3
        );
        
        let metal_value = (metal_pattern + imperfections * 0.5 + scratches * 0.3 + 1.5) / 3.0;
        
        let base_color = if metal_value > 0.8 {
            mix_color(chrome, bright_metal, (1.0 - metal_value) * 5.0)
        } else if metal_value > 0.6 {
            mix_color(bright_metal, light_metal, (0.8 - metal_value) * 5.0)
        } else if metal_value > 0.4 {
            mix_color(light_metal, medium_metal, (0.6 - metal_value) * 5.0)
        } else if metal_value > 0.2 {
            mix_color(medium_metal, dark_metal, (0.4 - metal_value) * 5.0)
        } else {
            mix_color(dark_metal, rust_accent, metal_value * 5.0)
        };
        
        // === ILUMINACIÓN METÁLICA ===
        let light_dir = uniforms.light_direction.normalize();
        let view_dir = (uniforms.camera_position - position).normalize();
         
        let diffuse = normal.dot(&light_dir).max(0.0) * 0.3;
        
        let reflect_dir = normal * (2.0 * normal.dot(&light_dir)) - light_dir;
        let specular = view_dir.dot(&reflect_dir).max(0.0).powf(32.0) * 1.2; 
        let specular_broad = view_dir.dot(&reflect_dir).max(0.0).powf(8.0) * 0.5;
        
        let fresnel = (1.0 - view_dir.dot(&normal).abs()).powf(3.0) * 0.4;
        let ambient = 0.2;
        let ao = (1.0 - imperfections.abs() * 0.3).max(0.5);
        
        let lighting_intensity = (ambient * ao + diffuse + specular + specular_broad + fresnel).min(2.0);
        
        let lit_color = ShaderColor::new(
            base_color.r * lighting_intensity,
            base_color.g * lighting_intensity,
            base_color.b * lighting_intensity,
            1.0,
        );
        

        let final_color = if specular > 0.8 {
            let highlight_amount = (specular - 0.8) * 5.0;
            mix_color(lit_color, chrome, highlight_amount.min(0.5))
        } else {
            lit_color
        };
        
        ShaderColor::new(
            final_color.r.clamp(0.0, 1.0),
            final_color.g.clamp(0.0, 1.0),
            final_color.b.clamp(0.0, 1.0),
            1.0,
        )
    }
}
