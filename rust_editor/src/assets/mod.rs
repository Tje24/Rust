// src/assets/mod.rs - Módulo de gestión de assets

mod asset_manager;

pub use asset_manager::AssetManager;

/// Tipo de asset
#[derive(Debug, Clone)]
pub enum AssetType {
    Mesh,
    Texture,
    Sound,
    Font,
    Scene,
}

/// Información de asset
#[derive(Debug, Clone)]
pub struct AssetInfo {
    pub id: uuid::Uuid,
    pub name: String,
    pub path: std::path::PathBuf,
    pub asset_type: AssetType,
    pub size_bytes: u64,
}

/// Configuración de compresión
#[derive(Debug, Clone)]
pub enum CompressionFormat {
    None,
    ETC2,
    ASTC,
    BC7,
}
