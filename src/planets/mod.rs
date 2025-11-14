// ============================================================================
// MÓDULO DE PLANETAS
// Cada planeta tiene su propia implementación en archivos separados
// ============================================================================

pub mod sun;
pub mod rocky;
pub mod gas_giant;
pub mod crystal;
pub mod nebula;
pub mod metallic;

// Re-exportar los shaders para facilitar su uso
pub use sun::SunShader;
pub use rocky::RockyPlanetShader;
pub use gas_giant::GasGiantShader;
pub use crystal::CrystalPlanetShader;
pub use nebula::LavaPlanetShader;
pub use metallic::SaturnShader;
