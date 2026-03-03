// src/tilemap/mod.rs - Módulo de tilemap

mod tilemap;
mod tileset;

pub use tilemap::Tilemap;
pub use tileset::Tileset;

/// Capa de tilemap
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TileLayer {
    pub id: uuid::Uuid,
    pub name: String,
    pub visible: bool,
    pub tiles: Vec<Vec<Option<TileInstance>>>,
}

/// Instancia de tile
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TileInstance {
    pub tile_id: u32,
    pub flip_x: bool,
    pub flip_y: bool,
}

/// Forma de colisión
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum CollisionShape {
    Rectangle { width: f32, height: f32 },
    Circle { radius: f32 },
}
