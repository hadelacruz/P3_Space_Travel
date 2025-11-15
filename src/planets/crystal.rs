use crate::vector::Vector3;
use crate::shaders::{ShaderColor, ShaderUniforms, PlanetShader, fbm, fbm3d, voronoi_noise, simple_noise, smoothstep, mix_color};

pub struct CrystalPlanetShader;

impl PlanetShader for CrystalPlanetShader {
    fn vertex_shader(&self, position: Vector3, normal: Vector3, _uv: (f32, f32), uniforms: &ShaderUniforms) -> (Vector3, Vector3) {
        // Deformación de pulso de energía
        let pulse = (uniforms.time * 3.0 + position.length() * 5.0).sin() * 0.01;
        let pulsed_position = position + normal * pulse;
        (pulsed_position, normal)
    }

    fn fragment_shader(&self, position: Vector3, normal: Vector3, uv: (f32, f32), uniforms: &ShaderUniforms) -> ShaderColor {
        // === PALETA TECNOLÓGICA ===
        let base_dark = ShaderColor::from_rgb(10, 15, 30);         
        let tech_blue = ShaderColor::from_rgb(0, 150, 255);        
        let cyber_cyan = ShaderColor::from_rgb(0, 255, 255);       
        let neon_green = ShaderColor::from_rgb(0, 255, 150);       
        let electric_purple = ShaderColor::from_rgb(150, 0, 255);  
        let hot_pink = ShaderColor::from_rgb(255, 0, 150);         
        let energy_white = ShaderColor::from_rgb(200, 255, 255);   
        let warning_orange = ShaderColor::from_rgb(255, 150, 0);   
        
        // === CAPA 1: GRILLA TECNOLÓGICA BASE ===
        let grid_size = 20.0;
        let grid_x = (uv.0 * grid_size).fract();
        let grid_y = (uv.1 * grid_size).fract();
        let grid_lines = smoothstep(0.02, 0.0, grid_x.min(1.0 - grid_x)) +
                        smoothstep(0.02, 0.0, grid_y.min(1.0 - grid_y));
        
        // === CAPA 2: CIRCUITOS HEXAGONALES ===
        let hex_pattern = voronoi_noise(uv.0 * 12.0, uv.1 * 12.0);
        let hex_cells = smoothstep(0.15, 0.2, hex_pattern);
        let hex_borders = smoothstep(0.18, 0.22, hex_pattern) - smoothstep(0.22, 0.25, hex_pattern);
        
        // === CAPA 3: FLUJO DE DATOS ===
        let data_flow1 = fbm(
            uv.0 * 15.0 + uniforms.time * 0.5,
            uv.1 * 15.0,
            4
        );
        let data_flow2 = fbm(
            uv.0 * 20.0 - uniforms.time * 0.7,
            uv.1 * 10.0 + uniforms.time * 0.3,
            3
        );
        let data_streams = smoothstep(0.6, 0.8, data_flow1) + smoothstep(0.65, 0.85, data_flow2);
        
        // === CAPA 4: PULSOS DE ENERGÍA ===
        let pulse_frequency = 5.0;
        let pulse_wave = (uniforms.time * pulse_frequency + position.length() * 3.0).sin() * 0.5 + 0.5;
        let pulse_wave2 = (uniforms.time * pulse_frequency * 1.5 - position.length() * 2.0).sin() * 0.5 + 0.5;
        let energy_pulse = pulse_wave * 0.6 + pulse_wave2 * 0.4;
        
        // === CAPA 5: NODOS DE PODER ===
        let power_nodes = voronoi_noise(uv.0 * 8.0, uv.1 * 8.0);
        let node_centers = smoothstep(0.08, 0.05, power_nodes);
        let node_glow = smoothstep(0.15, 0.05, power_nodes);
        
        // === CAPA 6: ESCANEO HOLOGRÁFICO ===
        let scan_line = (uv.1 * 10.0 - uniforms.time * 2.0) % 1.0;
        let scan_intensity = smoothstep(0.05, 0.0, (scan_line - 0.5).abs());
        
        // === CAPA 7: INTERFERENCIA DIGITAL ===
        let glitch = simple_noise(
            (uniforms.time * 10.0).floor() * 0.1,
            (uv.1 * 20.0).floor()
        );
        let glitch_effect = if glitch > 0.95 {
            simple_noise(uv.0 * 100.0 + uniforms.time * 50.0, uv.1) * 0.3
        } else {
            0.0
        };
        
        // === CONSTRUCCIÓN DEL COLOR ===
        let mut base_color = base_dark;
        
        // Agregar grilla base
        if grid_lines > 0.1 {
            base_color = mix_color(base_color, tech_blue, grid_lines * energy_pulse * 0.5);
        }
        
        // Agregar circuitos hexagonales
        if hex_borders > 0.1 {
            let circuit_color = mix_color(cyber_cyan, tech_blue, energy_pulse);
            base_color = mix_color(base_color, circuit_color, hex_borders * 0.8);
        }
        
        // Celdas hexagonales con variación de color
        if hex_cells > 0.5 {
            let cell_variety = simple_noise(uv.0 * 12.0, uv.1 * 12.0);
            let cell_color = if cell_variety > 0.7 {
                electric_purple
            } else if cell_variety > 0.4 {
                neon_green
            } else {
                tech_blue
            };
            base_color = mix_color(base_color, cell_color, hex_cells * 0.3 * energy_pulse);
        }
        
        // Agregar flujo de datos
        if data_streams > 0.5 {
            let stream_color = mix_color(neon_green, cyber_cyan, (uniforms.time * 2.0).sin() * 0.5 + 0.5);
            base_color = mix_color(base_color, stream_color, data_streams * 0.7);
        }
        
        // Agregar nodos de poder
        if node_centers > 0.5 {
            let node_pulse = (uniforms.time * 4.0 + uv.0 * 20.0).sin() * 0.5 + 0.5;
            let node_color = mix_color(hot_pink, energy_white, node_pulse);
            base_color = mix_color(base_color, node_color, node_centers);
        }
        
        if node_glow > 0.3 {
            base_color = mix_color(base_color, warning_orange, node_glow * 0.4 * energy_pulse);
        }
        
        // Agregar línea de escaneo
        if scan_intensity > 0.1 {
            base_color = mix_color(base_color, energy_white, scan_intensity * 0.8);
        }
        
        // Efecto de glitch
        if glitch_effect > 0.1 {
            let glitch_color = mix_color(hot_pink, cyber_cyan, glitch);
            base_color = mix_color(base_color, glitch_color, glitch_effect);
        }
        
        // === CAPA 8: PATRONES FRACTALES ===
        let fractal = fbm3d(
            position.x * 10.0 + uniforms.time * 0.1,
            position.y * 10.0,
            position.z * 10.0 - uniforms.time * 0.15,
            5
        );
        if fractal > 0.6 {
            let fractal_color = mix_color(electric_purple, tech_blue, fractal);
            base_color = mix_color(base_color, fractal_color, smoothstep(0.6, 0.75, fractal) * 0.4);
        }
        
        // === ILUMINACIÓN TECNOLÓGICA ===
        let light_dir = uniforms.light_direction.normalize();
        let view_dir = (uniforms.camera_position - position).normalize();
        
        let diffuse = normal.dot(&light_dir).max(0.0) * 0.2;
        
        let rim = (1.0 - view_dir.dot(&normal).abs()).powf(3.0);
        let rim_color = mix_color(cyber_cyan, hot_pink, (uniforms.time * 2.0).sin() * 0.5 + 0.5);
        
        let self_illumination = 0.8 + energy_pulse * 0.3;
        
        let ambient = 0.2;
        let lighting_intensity = (ambient + diffuse + self_illumination).min(2.0);
        
        let mut final_color = ShaderColor::new(
            base_color.r * lighting_intensity,
            base_color.g * lighting_intensity,
            base_color.b * lighting_intensity,
            1.0,
        );
        
        if rim > 0.4 {
            final_color = mix_color(final_color, rim_color, rim * 0.8);
        }
        
        ShaderColor::new(
            final_color.r.clamp(0.0, 1.0),
            final_color.g.clamp(0.0, 1.0),
            final_color.b.clamp(0.0, 1.0),
            1.0,
        )
    }
}
