// src/export/mod.rs - Módulo de exportación

mod exporter;

pub use exporter::ProjectExporter;

/// Configuración de exportación
#[derive(Debug, Clone)]
pub struct ExportConfig {
    pub output_path: String,
    pub package_name: String,
    pub version_name: String,
    pub version_code: u32,
    pub architectures: Vec<Architecture>,
}

/// Arquitectura de destino
#[derive(Debug, Clone)]
pub enum Architecture {
    Arm64V8a,
    ArmeabiV7a,
    X86_64,
}

/// Error de exportación
#[derive(Debug, thiserror::Error)]
pub enum ExportError {
    #[error("Error de compilación: {0}")]
    Compilation(String),
    
    #[error("Error de empaquetado: {0}")]
    Packaging(String),
    
    #[error("Error de firma: {0}")]
    Signing(String),
}
