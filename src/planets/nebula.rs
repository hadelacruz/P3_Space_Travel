use crate::vector::Vector3;
use crate::shaders::{ShaderColor, ShaderUniforms, PlanetShader, fbm, fbm3d, voronoi_noise, ridge_noise, simple_noise, smoothstep, mix_color};

pub struct NebulaPlanetShader;

impl PlanetShader for NebulaPlanetShader {
    fn vertex_shader(&self, position: Vector3, normal: Vector3, _uv: (f32, f32), uniforms: &ShaderUniforms) -> (Vector3, Vector3) {
        let wave1 = (uniforms.time * 1.5 + position.x * 3.0 + position.y * 2.0).sin() * 0.03;
        let wave2 = (uniforms.time * 2.0 - position.z * 4.0 + position.y).cos() * 0.02;
        let wavy_position = position + normal * (wave1 + wave2);
        (wavy_position, normal)
    }

    fn fragment_shader(&self, position: Vector3, normal: Vector3, uv: (f32, f32), uniforms: &ShaderUniforms) -> ShaderColor {
        // === PALETA NEBULOSA CÓSMICA ===
        let void_black = ShaderColor::from_rgb(5, 0, 10);          
        let deep_purple = ShaderColor::from_rgb(30, 0, 60);        
        let royal_purple = ShaderColor::from_rgb(75, 0, 130);      
        let magenta = ShaderColor::from_rgb(200, 0, 150);          
        let hot_pink = ShaderColor::from_rgb(255, 20, 147);        
        let electric_blue = ShaderColor::from_rgb(0, 100, 255);    
        let cyan_bright = ShaderColor::from_rgb(0, 255, 255);      
        let orange_flame = ShaderColor::from_rgb(255, 100, 0);     
        let yellow_star = ShaderColor::from_rgb(255, 255, 100);    
        let white_hot = ShaderColor::from_rgb(255, 255, 255);     
        
        // === CAPA 1: GAS NEBULAR BASE 
        let nebula_gas1 = fbm3d(
            position.x * 2.0 + uniforms.time * 0.03,
            position.y * 2.0 + uniforms.time * 0.02,
            position.z * 2.0 - uniforms.time * 0.025,
            7
        );
        let nebula_gas2 = fbm3d(
            position.x * 3.0 - uniforms.time * 0.02,
            position.y * 3.0 + uniforms.time * 0.035,
            position.z * 3.0 + uniforms.time * 0.015,
            6
        );
        let nebula_density = nebula_gas1 * 0.6 + nebula_gas2 * 0.4;
        
        // === CAPA 2: REMOLINOS DE POLVO CÓSMICO ===
        let cosmic_dust1 = fbm(
            uv.0 * 8.0 + uniforms.time * 0.05,
            uv.1 * 8.0 - uniforms.time * 0.04,
            5
        );
        let cosmic_dust2 = fbm(
            uv.0 * 12.0 - uniforms.time * 0.03,
            uv.1 * 12.0 + uniforms.time * 0.06,
            4
        );
        let dust_swirls = cosmic_dust1 * 0.5 + cosmic_dust2 * 0.5;
        
        // === CAPA 3: CAMPOS DE IONIZACIÓN ===
        let ionization1 = fbm3d(
            position.x * 5.0 + uniforms.time * 0.1,
            position.y * 5.0,
            position.z * 5.0 - uniforms.time * 0.08,
            4
        );
        let ionization2 = fbm3d(
            position.x * 7.0 - uniforms.time * 0.12,
            position.y * 7.0 + uniforms.time * 0.09,
            position.z * 7.0,
            3
        );
        let ion_fields = ionization1 * 0.6 + ionization2 * 0.4;
        
        // === CAPA 4: VÓRTICES MAGNÉTICOS ===
        let vortex_pattern = voronoi_noise(
            uv.0 * 6.0 + uniforms.time * 0.04,
            uv.1 * 6.0 - uniforms.time * 0.035
        );
        let vortex_intensity = smoothstep(0.2, 0.1, vortex_pattern);
        
        // === CAPA 5: RAYOS CÓSMICOS Y RADIACIÓN ===
        let cosmic_rays = ridge_noise(
            uv.0 * 20.0 + uniforms.time * 0.3,
            uv.1 * 20.0 - uniforms.time * 0.25,
            3
        );
        let ray_intensity = smoothstep(0.75, 0.9, cosmic_rays);
        
        // === CAPA 6: ESTRELLAS EN FORMACIÓN ===
        let star_formation = voronoi_noise(uv.0 * 15.0, uv.1 * 15.0);
        let proto_stars = smoothstep(0.05, 0.02, star_formation);
        let star_glow = smoothstep(0.12, 0.02, star_formation);
        
        // === CAPA 7: PULSOS DE ENERGÍA CÓSMICA ===
        let energy_pulse1 = (uniforms.time * 2.0 + position.length() * 3.0).sin() * 0.5 + 0.5;
        let energy_pulse2 = (uniforms.time * 3.0 - position.length() * 2.0).cos() * 0.5 + 0.5;
        let cosmic_pulse = energy_pulse1 * 0.6 + energy_pulse2 * 0.4;
        
        // === CAPA 8: ONDAS DE CHOQUE ===
        let shockwave_distance = ((position.x + uniforms.time * 0.5).powi(2) + 
                                  position.y.powi(2) + 
                                  position.z.powi(2)).sqrt();
        let shockwave = (shockwave_distance * 5.0 - uniforms.time * 3.0).sin() * 0.5 + 0.5;
        let shockwave_intensity = smoothstep(0.7, 0.9, shockwave) * smoothstep(0.3, 0.4, nebula_density);
        
        // === CONSTRUCCIÓN DEL COLOR BASE ===
        let mut base_color = void_black;
        
        // Gradiente de gas nebular
        if nebula_density > 0.2 {
            let gas_color = if nebula_density > 0.7 {
                mix_color(royal_purple, magenta, smoothstep(0.7, 0.85, nebula_density))
            } else if nebula_density > 0.5 {
                mix_color(deep_purple, royal_purple, smoothstep(0.5, 0.7, nebula_density))
            } else {
                mix_color(void_black, deep_purple, smoothstep(0.2, 0.5, nebula_density))
            };
            base_color = mix_color(base_color, gas_color, 0.9);
        }
        
        if dust_swirls > 0.55 {
            let dust_color = mix_color(orange_flame, yellow_star, smoothstep(0.55, 0.75, dust_swirls));
            let dust_alpha = smoothstep(0.55, 0.7, dust_swirls) * 0.6;
            base_color = mix_color(base_color, dust_color, dust_alpha);
        }
        
        if ion_fields > 0.6 {
            let ion_color = mix_color(electric_blue, cyan_bright, cosmic_pulse);
            let ion_alpha = smoothstep(0.6, 0.75, ion_fields) * 0.7;
            base_color = mix_color(base_color, ion_color, ion_alpha);
        }
        
        if vortex_intensity > 0.4 {
            let vortex_color = mix_color(hot_pink, magenta, cosmic_pulse);
            base_color = mix_color(base_color, vortex_color, vortex_intensity * 0.8);
        }
        
        if ray_intensity > 0.5 {
            let ray_color = mix_color(cyan_bright, white_hot, energy_pulse1);
            base_color = mix_color(base_color, ray_color, ray_intensity * 0.9);
        }
        
        if proto_stars > 0.6 {
            let star_pulse = (uniforms.time * 5.0 + uv.0 * 50.0).sin() * 0.5 + 0.5;
            let star_color = mix_color(yellow_star, white_hot, star_pulse);
            base_color = mix_color(base_color, star_color, proto_stars);
        }
        
        if star_glow > 0.3 && star_glow < 0.7 {
            let glow_color = mix_color(orange_flame, yellow_star, cosmic_pulse);
            base_color = mix_color(base_color, glow_color, star_glow * 0.5);
        }
        
        if shockwave_intensity > 0.5 {
            let shock_color = mix_color(cyan_bright, electric_blue, shockwave);
            base_color = mix_color(base_color, shock_color, shockwave_intensity * 0.7);
        }
        
        // === CAPA 9: BRILLO VOLUMÉTRICO ===
        let volumetric_glow = nebula_density * 0.5 + dust_swirls * 0.3 + ion_fields * 0.2;
        
        // === CAPA 10: PARTÍCULAS ESTELARES ===
        let particles = simple_noise(uv.0 * 100.0, uv.1 * 100.0);
        if particles > 0.98 {
            let particle_brightness = simple_noise(uv.0 * 200.0 + uniforms.time, uv.1 * 200.0);
            let particle_color = mix_color(yellow_star, white_hot, particle_brightness);
            base_color = mix_color(base_color, particle_color, (particles - 0.98) * 50.0);
        }
        
        // === ILUMINACIÓN VOLUMÉTRICA ===
        let light_dir = uniforms.light_direction.normalize();
        let view_dir = (uniforms.camera_position - position).normalize();
        
        let diffuse = normal.dot(&light_dir).max(0.0) * 0.1;
        let self_illumination = 1.2 + cosmic_pulse * 0.5 + volumetric_glow * 0.8;
        let rim = (1.0 - view_dir.dot(&normal).abs()).powf(2.0);
        let rim_color = mix_color(
            mix_color(magenta, cyan_bright, energy_pulse1),
            hot_pink,
            energy_pulse2
        );
        
        let internal_scatter = (1.0 - nebula_density.abs()) * 0.3;
        
        let ambient = 0.1;
        let lighting_intensity = (ambient + diffuse + self_illumination + internal_scatter).min(2.5);
        
        let mut final_color = ShaderColor::new(
            base_color.r * lighting_intensity,
            base_color.g * lighting_intensity,
            base_color.b * lighting_intensity,
            1.0,
        );
        
        if rim > 0.3 {
            final_color = mix_color(final_color, rim_color, rim * 0.7);
        }
        
        if volumetric_glow > 0.6 {
            let bloom_intensity = smoothstep(0.6, 0.8, volumetric_glow) * 0.3;
            let bloom_color = ShaderColor::from_rgb(255, 200, 255);
            final_color = mix_color(final_color, bloom_color, bloom_intensity);
        }
        
        ShaderColor::new(
            final_color.r.clamp(0.0, 1.0),
            final_color.g.clamp(0.0, 1.0),
            final_color.b.clamp(0.0, 1.0),
            1.0,
        )
    }
}
