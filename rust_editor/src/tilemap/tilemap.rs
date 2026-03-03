// src/tilemap/tilemap.rs - Estructura de tilemap

use uuid::Uuid;
use super::TileLayer;

/// Tilemap
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Tilemap {
    pub id: Uuid,
    pub name: String,
    pub tileset_id: Uuid,
    pub grid_size: [u32; 2],
    pub tile_size: [u32; 2],
    pub layers: Vec<TileLayer>,
}

impl Default for Tilemap {
    fn default() -> Self {
        Tilemap {
            id: Uuid::new_v4(),
            name: "New Tilemap".to_string(),
            tileset_id: Uuid::nil(),
            grid_size: [32, 32],
            tile_size: [32, 32],
            layers: Vec::new(),
        }
    }
}

impl Tilemap {
    /// Obtener tile en una posición
    pub fn get_tile(&self, layer: usize, x: u32, y: u32) -> Option<&super::TileInstance> {
        self.layers.get(layer)
            .and_then(|l| l.tiles.get(y as usize))
            .and_then(|row| row.get(x as usize))
            .and_then(|tile| tile.as_ref())
    }
    
    /// Establecer tile en una posición
    pub fn set_tile(&mut self, layer: usize, x: u32, y: u32, tile: Option<super::TileInstance>) {
        if let Some(layer) = self.layers.get_mut(layer) {
            if let Some(row) = layer.tiles.get_mut(y as usize) {
                if let Some(cell) = row.get_mut(x as usize) {
                    *cell = tile;
                }
            }
        }
    }
}
