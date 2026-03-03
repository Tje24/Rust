// src/assets/asset_manager.rs - Gestor de assets

use super::{AssetInfo, AssetType, CompressionFormat};
use std::collections::HashMap;
use std::path::PathBuf;

/// Gestor de assets
pub struct AssetManager {
    pub assets: HashMap<uuid::Uuid, AssetInfo>,
    pub memory_budget_mb: usize,
    pub compression: CompressionFormat,
}

impl AssetManager {
    pub fn new(memory_budget_mb: usize) -> Self {
        AssetManager {
            assets: HashMap::new(),
            memory_budget_mb,
            compression: CompressionFormat::ETC2,
        }
    }
    
    pub fn load_asset(&mut self, path: &str, asset_type: AssetType) -> Result<uuid::Uuid, anyhow::Error> {
        let path_buf = PathBuf::from(path);
        let file_size = std::fs::metadata(&path_buf)?.len();
        
        let id = uuid::Uuid::new_v4();
        
        let info = AssetInfo {
            id,
            name: path_buf.file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            path: path_buf,
            asset_type,
            size_bytes: file_size,
        };
        
        self.assets.insert(id, info);
        
        log::info!("Asset cargado: {} ({:?})", path, id);
        
        Ok(id)
    }
    
    pub fn unload_asset(&mut self, id: uuid::Uuid) {
        self.assets.remove(&id);
        log::debug!("Asset descargado: {:?}", id);
    }
    
    pub fn get_asset(&self, id: uuid::Uuid) -> Option<&AssetInfo> {
        self.assets.get(&id)
    }
    
    pub fn get_memory_usage(&self) -> u64 {
        self.assets.values().map(|a| a.size_bytes).sum()
    }
}

impl Default for AssetManager {
    fn default() -> Self {
        Self::new(500) // 500 MB por defecto
    }
}
