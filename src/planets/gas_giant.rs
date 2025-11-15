use crate::vector::Vector3;
use crate::shaders::{ShaderColor, ShaderUniforms, PlanetShader, fbm, smoothstep, mix_color};

pub struct GasPlanetShader;

impl PlanetShader for GasPlanetShader {
    fn vertex_shader(&self, position: Vector3, normal: Vector3, _uv: (f32, f32), _uniforms: &ShaderUniforms) -> (Vector3, Vector3) {
        (position, normal)
    }

    fn fragment_shader(&self, _position: Vector3, normal: Vector3, uv: (f32, f32), uniforms: &ShaderUniforms) -> ShaderColor {
        // === PALETA DE JÚPITER - MUY CONTRASTADA ===
        let very_dark = ShaderColor::from_rgb(60, 35, 15);         
        let dark_brown = ShaderColor::from_rgb(110, 65, 25);    
        let rust_brown = ShaderColor::from_rgb(150, 85, 35);      
        let orange = ShaderColor::from_rgb(200, 120, 50);          
        let tan = ShaderColor::from_rgb(210, 150, 90);             
        let beige = ShaderColor::from_rgb(230, 190, 130);          
        let cream = ShaderColor::from_rgb(245, 220, 170);          
        let white = ShaderColor::from_rgb(255, 250, 230);         
        let red_spot = ShaderColor::from_rgb(200, 60, 30);         
        
        let latitude = uv.1;
        let animated_longitude = uv.0 + uniforms.time * 0.015;
        
        // === BANDAS BASE CON TEXTURA ===
        let band_pos = latitude * 14.0;
        let band_index = band_pos.floor() as i32 % 8;
        let band_fract = band_pos.fract();
        
        let jet_stream = fbm(
            animated_longitude * 12.0,
            latitude * 5.0,
            5
        ) * 0.25;
        
        // Remolinos y vórtices a lo largo de las bandas
        let vortices = fbm(
            animated_longitude * 8.0 + latitude * 20.0,
            latitude * 8.0,
            4
        ) * 0.15;
        
        // Textura fina
        let fine_texture = fbm(
            animated_longitude * 25.0,
            latitude * 20.0,
            3
        ) * 0.08;
        
        // Combinar todas las texturas
        let texture_value = jet_stream + vortices + fine_texture;
        let final_band = (band_fract + texture_value).clamp(0.0, 1.0);
        
        // === COLORES POR BANDA (Alternando oscuro/claro) ===
        let base_color = match band_index {
            0 => mix_color(very_dark, dark_brown, final_band),
            1 => mix_color(tan, beige, final_band),
            2 => mix_color(dark_brown, rust_brown, final_band),
            3 => mix_color(cream, white, final_band),
            4 => mix_color(rust_brown, orange, final_band),
            5 => mix_color(beige, cream, final_band),
            6 => mix_color(orange, tan, final_band),
            _ => mix_color(white, cream, final_band),
        };
        
        let mut final_color = base_color;
        
        let storm_x = 0.3;
        let storm_y = 0.4;
        let dx = (animated_longitude - storm_x) * 2.5; 
        let dy = latitude - storm_y;
        let dist_storm = (dx * dx + dy * dy).sqrt();
        
        if dist_storm < 0.15 {
            let strength = smoothstep(0.15, 0.06, dist_storm);
            let angle = dy.atan2(dx);
            let spiral = (angle * 3.0 - dist_storm * 10.0 + uniforms.time * 0.5).sin() * 0.5 + 0.5;
            let storm_color = mix_color(red_spot, orange, spiral);
            final_color = mix_color(final_color, storm_color, strength * 0.9);
        }
        
        // === ÓVALOS BLANCOS (tormentas menores) ===
        let oval1_dist = ((animated_longitude - 0.6).powi(2) * 4.0 + (latitude - 0.55).powi(2)).sqrt();
        if oval1_dist < 0.05 {
            let oval_str = smoothstep(0.05, 0.02, oval1_dist);
            final_color = mix_color(final_color, white, oval_str * 0.8);
        }
        
        let oval2_dist = ((animated_longitude - 0.75).powi(2) * 5.0 + (latitude - 0.32).powi(2)).sqrt();
        if oval2_dist < 0.04 {
            let oval_str = smoothstep(0.04, 0.015, oval2_dist);
            final_color = mix_color(final_color, cream, oval_str * 0.7);
        }
        
        // === ILUMINACIÓN ===
        let light_dir = uniforms.light_direction.normalize();
        let diffuse = normal.dot(&light_dir).max(0.0);
        let ambient = 0.4;
        let lighting = (ambient + diffuse * 0.6).min(1.0);
        
        ShaderColor::new(
            (final_color.r * lighting).clamp(0.0, 1.0),
            (final_color.g * lighting).clamp(0.0, 1.0),
            (final_color.b * lighting).clamp(0.0, 1.0),
            1.0,
        )
    }
}
