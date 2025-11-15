use crate::vector::Vector3;
use crate::shaders::{ShaderColor, ShaderUniforms, PlanetShader, fbm, fbm3d, voronoi_noise, smoothstep, mix_color};

pub struct MetallicPlanetShader;

impl PlanetShader for MetallicPlanetShader {
    fn vertex_shader(&self, position: Vector3, normal: Vector3, _uv: (f32, f32), uniforms: &ShaderUniforms) -> (Vector3, Vector3) {
        // === GENERAR PICOS/PÚAS PROCEDURALMENTE ===
        
        // CAPA 1: Picos grandes principales
        let voronoi_scale = 15.0;
        let voronoi_pattern = voronoi_noise(
            position.x * voronoi_scale + position.z * voronoi_scale,
            position.y * voronoi_scale
        );
        
        // Los picos se generan donde el Voronoi es pequeño (centros de células)
        let spike_large = if voronoi_pattern < 0.15 {
            smoothstep(0.15, 0.05, voronoi_pattern) * 0.35 
        } else {
            0.0
        };
        
        // CAPA 2: Picos medianos (más densidad)
        let voronoi_medium = voronoi_noise(
            position.x * 25.0 + position.z * 25.0,
            position.y * 25.0
        );
        
        let spike_medium = if voronoi_medium < 0.12 {
            smoothstep(0.12, 0.04, voronoi_medium) * 0.25
        } else {
            0.0
        };
        
        // CAPA 3: Picos pequeños 
        let voronoi_small = voronoi_noise(
            position.x * 40.0 + uniforms.time * 0.1 + position.z * 40.0,
            position.y * 40.0
        );
        
        let spike_small = if voronoi_small < 0.1 {
            smoothstep(0.1, 0.03, voronoi_small) * 0.15
        } else {
            0.0
        };
        
        // CAPA 4: Rugosidad base
        let roughness = fbm3d(
            position.x * 50.0,
            position.y * 50.0,
            position.z * 50.0,
            4
        ) * 0.05;
        
        // CAPA 5: Deformación animada (pulsación metálica)
        let pulse = (uniforms.time * 2.0 + position.length() * 5.0).sin() * 0.02;
        
        let total_displacement = spike_large + spike_medium + spike_small + roughness + pulse;        
        let displaced_position = position + normal * total_displacement;
        let spike_factor = spike_large * 2.0 + spike_medium * 1.5 + spike_small;
        let adjusted_normal = (normal + normal * spike_factor).normalize();
        
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
        
        // Combinar para obtener color base metálico
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
        
        // Difusa 
        let diffuse = normal.dot(&light_dir).max(0.0) * 0.3;
        
        // Especular FUERTE 
        let reflect_dir = normal * (2.0 * normal.dot(&light_dir)) - light_dir;
        let specular = view_dir.dot(&reflect_dir).max(0.0).powf(32.0) * 1.2; 
        let specular_broad = view_dir.dot(&reflect_dir).max(0.0).powf(8.0) * 0.5;
        
        // Fresnel effect 
        let fresnel = (1.0 - view_dir.dot(&normal).abs()).powf(3.0) * 0.4;
        
        // Ambiente metálico
        let ambient = 0.2;
        
        // Oclusión ambiental en los valles entre picos
        let ao = (1.0 - imperfections.abs() * 0.3).max(0.5);
        
        // Combinar iluminación
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
