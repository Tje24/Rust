// src/tilemap/tileset.rs - Tileset de sprites

use uuid::Uuid;

/// Tileset
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Tileset {
    pub id: Uuid,
    pub name: String,
    pub texture_id: Uuid,
    pub tile_size: [u32; 2],
    pub tiles: Vec<TileDefinition>,
    pub margin: u32,
    pub spacing: u32,
}

/// Definición de tile
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TileDefinition {
    pub id: u32,
    pub name: String,
    pub rect: [u32; 4], // x, y, width, height
    pub collision: Option<super::CollisionShape>,
}

impl Default for Tileset {
    fn default() -> Self {
        Tileset {
            id: Uuid::new_v4(),
            name: "New Tileset".to_string(),
            texture_id: Uuid::nil(),
            tile_size: [32, 32],
            tiles: Vec::new(),
            margin: 0,
            spacing: 0,
        }
    }
}
