# Proyecto 3- Space Travel 🌌

Simulación de un sistema solar procedural usando un software renderer diseñado desde cero en Rust.

## Video

## https://youtu.be/bmLrgTaGQrE ##

## Características Implementadas ✨

### Requerimientos del Proyecto

- ✅ **Sol Central**: Estrella con efectos de plasma, llamaradas y emisión de luz
- ✅ **Múltiples Planetas**: 5 planetas con shaders únicos y procedurales
- ✅ **Plano Eclíptico**: Todos los planetas orbitan en un plano común
- ✅ **Órbitas Circulares**: Cada planeta se traslada en su órbita
- ✅ **Rotación sobre el Eje**: Todos los cuerpos rotan sobre sí mismos
- ✅ **Cámara Móvil**: Control completo de la cámara en el plano eclíptico
- ✅ **Shaders Procedurales**: Cada planeta tiene su shader único
- ✅ **Skybox Estelar**: Fondo con ~800 estrellas procedurales que parpadean

### Planetas del Sistema

1. **Sol** 
   - Superficie animada con plasma
   - Efectos de llamaradas solares
   - Prominencias procedurales
   - Pulsación de intensidad

2. **Planeta Rocoso** 
   - Terreno con relieve procedural
   - Montañas, colinas y cráteres
   - Texturas grises realistas

3. **Gigante Gaseoso** 
   - Bandas atmosféricas animadas
   - Colores vibrantes
   - Tormentas procedurales

4. **Planeta de Cristal** 
   - Superficies cristalinas
   - Efectos de refracción
   - Colores brillantes

5. **Planeta Nebulosa** 
   - Superficie volcánica
   - Flujos de lava animados
   - Efectos de calor

6. **Planeta Metalico** 
   - Sistema de anillos
   - Atmósfera gaseosa
   - Colores característicos

## Controles 🎮

### Cámara
- **←/→ (Flechas)**: Rotar alrededor del sistema solar
- **↑/↓ (Flechas)**: Acercar/Alejar zoom
- **W/S**: Subir/Bajar altura de la cámara
- **ESC**: Salir de la aplicación

## Compilación y Ejecución 🚀

### Requisitos
- Rust 1.70 o superior
- Cargo

### Compilar
```bash
cargo build --release
```

### Ejecutar
```bash
cargo run --release
```

## Arquitectura Técnica 🏗️

### Software Renderer
El proyecto implementa un renderer completamente desde cero con:
- **Rasterización de triángulos**: Algoritmo de edge function
- **Z-Buffer**: Para resolver visibilidad
- **Renderizado de líneas**: Algoritmo de Bresenham para el plano eclíptico
- **Skybox Procedural**: Campo de estrellas con ~800 estrellas distribuidas uniformemente
- **Parpadeo de Estrellas**: Efecto de twinkle con variación de brillo en tiempo real
- **Plano Eclíptico Visual**: Cuadrícula 3D que muestra el plano orbital
- **Círculos Orbitales**: Visualización de las trayectorias de cada planeta
- **Ejes de Coordenadas**: Sistema de referencia RGB (X=Rojo, Y=Verde, Z=Azul)
- **Vertex Shaders**: Deformación procedural de geometría
- **Fragment Shaders**: Colores y efectos procedurales
- **Transformaciones 3D**: Matrices de modelo, vista y proyección



## Estructura del Proyecto 📁

```
src/
├── main.rs              # Renderer principal y loop del juego
├── vector.rs            # Matemáticas vectoriales
├── shaders.rs           # Sistema de shaders y utilidades
├── framebuffer.rs       # Buffer de color y profundidad
├── obj_loader.rs        # Cargador de modelos .obj
├── skybox.rs            # Renderizado de estrellas de fondo
├── matrix.rs 
├── planet.rs  
├── render.rs             
└── planets/
    ├── mod.rs          # Módulo de planetas
    ├── sun.rs          # Shader del sol
    ├── rocky.rs        # Shader planeta rocoso
    ├── gas_giant.rs    # Shader gigante gaseoso
    ├── crystal.rs      # Shader planeta cristalino
    ├── nebula.rs       # Shader planeta de lava
    └── metallic.rs     # Shader planeta con anillos
```


