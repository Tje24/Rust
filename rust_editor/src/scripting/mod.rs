// src/scripting/mod.rs - Módulo de scripting basado en TOML/JSON

mod scene;
mod node;
mod components;
mod behavior;

pub use scene::Scene;
pub use node::Node;
pub use components::*;
pub use behavior::*;

/// Metadata de la escena
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct SceneMetadata {
    pub name: String,
    pub author: String,
    pub version: String,
    pub created: String,
    pub last_modified: String,
    pub description: String,
}

/// Configuración de físicas
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PhysicsSettings {
    pub gravity: [f32; 3],
    pub physics_rate: u32,
}

impl Default for PhysicsSettings {
    fn default() -> Self {
        PhysicsSettings {
            gravity: [0.0, -9.8, 0.0],
            physics_rate: 60,
        }
    }
}

/// Assets de la escena
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct SceneAssets {
    pub meshes: Vec<AssetInfo>,
    pub textures: Vec<AssetInfo>,
}

/// Información de un asset
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AssetInfo {
    pub id: String,
    pub path: String,
    #[serde(default)]
    pub scale: f32,
}
