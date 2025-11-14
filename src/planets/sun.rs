// ============================================================================
// SOL: ESTRELLA CON EFECTOS DE FUEGO Y EMISIÓN DE LUZ
// Características: Superficie animada con plasma, emisión de luz intensa, colores cálidos
// ============================================================================

use crate::vector::Vector3;
use crate::shaders::{ShaderColor, ShaderUniforms, PlanetShader, fbm3d, fbm, mix_color, smoothstep};

pub struct SunShader;

impl PlanetShader for SunShader {
    fn vertex_shader(&self, position: Vector3, normal: Vector3, _uv: (f32, f32), uniforms: &ShaderUniforms) -> (Vector3, Vector3) {
        // === DEFORMACIÓN SUTIL DE SUPERFICIE SOLAR (MÁS ESFÉRICO) ===
        
        // Capa 1: Ondas de plasma muy sutiles (escala grande)
        let plasma_wave = fbm3d(
            position.x * 1.5 + uniforms.time * 0.2,
            position.y * 1.5 + uniforms.time * 0.15,
            position.z * 1.5,
            3
        ) * 0.02;  // Reducido de 0.08 a 0.02
        
        // Capa 2: Turbulencia mínima (llamaradas sutiles)
        let flares = fbm3d(
            position.x * 2.5 + uniforms.time * 0.3,
            position.y * 2.5 - uniforms.time * 0.25,
            position.z * 2.5 + uniforms.time * 0.1,
            2
        ) * 0.025;  // Reducido de 0.12 a 0.025
        
        // Capa 3: Detalles muy finos de superficie (granulación)
        let granulation = fbm3d(
            position.x * 8.0 + uniforms.time * 0.5,
            position.y * 8.0 + uniforms.time * 0.4,
            position.z * 8.0,
            2
        ) * 0.015;  // Reducido de 0.04 a 0.015
        
        // Capa 4: Prominencias solares ocasionales (muy reducidas)
        let prominence_noise = fbm3d(
            position.x * 1.5,
            position.y * 1.5 + uniforms.time * 0.4,
            position.z * 1.5,
            2
        );
        
        let prominence = if prominence_noise > 0.7 {
            (prominence_noise - 0.7) * 0.08 * (uniforms.time * 1.5).sin().abs()
        } else {
            0.0
        };  // Reducido de 0.3 a 0.08
        
        // Combinar todas las deformaciones (mucho más sutiles)
        let total_displacement = plasma_wave + flares + granulation + prominence;
        
        // Aplicar deformación mínima a lo largo de la normal
        let deformed_position = Vector3::new(
            position.x + normal.x * total_displacement,
            position.y + normal.y * total_displacement,
            position.z + normal.z * total_displacement,
        );
        
        // Mantener la normal casi original para mayor esfericidad
        let epsilon = 0.01;
        let neighbor_noise = fbm3d(
            (position.x + epsilon) * 1.5 + uniforms.time * 0.2,
            (position.y + epsilon) * 1.5 + uniforms.time * 0.15,
            (position.z + epsilon) * 1.5,
            3
        ) * 0.02;
        
        let tangent_displacement = neighbor_noise - plasma_wave;
        let normal_perturbation = Vector3::new(
            -tangent_displacement,
            -tangent_displacement,
            -tangent_displacement,
        ) * 0.2;  // Reducido de 0.5 a 0.2
        
        let perturbed_normal = Vector3::new(
            normal.x + normal_perturbation.x,
            normal.y + normal_perturbation.y,
            normal.z + normal_perturbation.z,
        ).normalize();
        
        (deformed_position, perturbed_normal)
    }

    fn fragment_shader(&self, position: Vector3, normal: Vector3, _uv: (f32, f32), uniforms: &ShaderUniforms) -> ShaderColor {
        // === PALETA DE COLORES DEL SOL ===
        let core_white = ShaderColor::from_rgb(255, 255, 255);      // Núcleo blanco brillante
        let bright_yellow = ShaderColor::from_rgb(255, 250, 200);   // Amarillo brillante
        let _yellow = ShaderColor::from_rgb(255, 220, 100);         // Amarillo (reservado)
        let orange = ShaderColor::from_rgb(255, 150, 50);           // Naranja
        let red_orange = ShaderColor::from_rgb(255, 100, 30);       // Naranja rojizo
        let deep_red = ShaderColor::from_rgb(200, 50, 20);          // Rojo profundo
        
        // === RUIDO PROCEDURAL PARA PATRONES DE PLASMA ===
        
        // Ruido de plasma principal (animado)
        let plasma_noise = fbm3d(
            position.x * 2.0 + uniforms.time * 0.4,
            position.y * 2.0 - uniforms.time * 0.3,
            position.z * 2.0 + uniforms.time * 0.1,
            5
        );
        
        // Ruido secundario para variación
        let detail_noise = fbm3d(
            position.x * 5.0 + uniforms.time * 0.6,
            position.y * 5.0 + uniforms.time * 0.5,
            position.z * 5.0,
            4
        );
        
        // Ruido de llamaradas (más rápido)
        let flare_noise = fbm3d(
            position.x * 3.0 + uniforms.time * 1.0,
            position.y * 3.0 - uniforms.time * 0.8,
            position.z * 3.0,
            3
        );
        
        // Ruido para manchas solares (más lentas)
        let sunspot_noise = fbm(
            position.x * 4.0 + uniforms.time * 0.1,
            position.y * 4.0,
            3
        );
        
        // === MEZCLA DE COLORES BASADA EN RUIDO ===
        
        // Color base: transición de núcleo a superficie
        let base_mix = smoothstep(-0.3, 0.3, plasma_noise);
        let mut color = mix_color(core_white, bright_yellow, base_mix);
        
        // Añadir capas de naranja y rojo
        let orange_mix = smoothstep(0.2, 0.6, plasma_noise + detail_noise * 0.5);
        color = mix_color(color, orange, orange_mix);
        
        let red_mix = smoothstep(0.5, 0.8, plasma_noise + detail_noise * 0.3);
        color = mix_color(color, red_orange, red_mix);
        
        // Añadir regiones más oscuras (manchas solares)
        if sunspot_noise < 0.2 {
            let sunspot_intensity = 1.0 - sunspot_noise / 0.2;
            color = mix_color(color, deep_red, sunspot_intensity * 0.4);
        }
        
        // Añadir llamaradas brillantes
        if flare_noise > 0.7 {
            let flare_intensity = (flare_noise - 0.7) / 0.3;
            color = mix_color(color, bright_yellow, flare_intensity * 0.6);
        }
        
        // === ILUMINACIÓN (el sol emite luz, no la recibe) ===
        // No aplicamos iluminación normal, el sol brilla por sí mismo
        
        // Añadir un poco de brillo en los bordes (limb brightening/darkening)
        let view_dir = (uniforms.camera_position - position).normalize();
        let fresnel = 1.0 - view_dir.dot(&normal).abs();
        let edge_glow = fresnel.powf(3.0);
        
        // Los bordes son ligeramente más brillantes
        color = mix_color(color, core_white, edge_glow * 0.3);
        
        // === PULSACIÓN DE INTENSIDAD ===
        let pulse = ((uniforms.time * 1.5).sin() + 1.0) * 0.5; // Oscila entre 0 y 1
        let pulse_intensity = 0.9 + pulse * 0.1; // Oscila entre 0.9 y 1.0
        
        color.r = (color.r * pulse_intensity).min(1.0);
        color.g = (color.g * pulse_intensity).min(1.0);
        color.b = (color.b * pulse_intensity).min(1.0);
        
        color
    }
}
