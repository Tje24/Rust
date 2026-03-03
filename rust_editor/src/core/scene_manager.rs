// src/core/scene_manager.rs - Gestor de escenas

use std::path::PathBuf;
use uuid::Uuid;

use crate::scripting::Scene;

/// Gestor de escenas del editor
pub struct SceneManager {
    /// Escena actualmente cargada
    pub current_scene: Option<Scene>,
    
    /// Ruta de la escena actual
    pub scene_path: Option<PathBuf>,
    
    /// ¿Hay cambios sin guardar?
    pub modified: bool,
    
    /// Historial de undo/redo
    pub undo_stack: Vec<SceneSnapshot>,
    pub redo_stack: Vec<SceneSnapshot>,
}

/// Snapshot para undo/redo
pub struct SceneSnapshot {
    pub scene: Scene,
    pub timestamp: std::time::Instant,
    pub description: String,
}

impl SceneManager {
    /// Crear nuevo gestor de escenas
    pub fn new() -> Self {
        SceneManager {
            current_scene: None,
            scene_path: None,
            modified: false,
            undo_stack: Vec::with_capacity(50),
            redo_stack: Vec::new(),
        }
    }
    
    /// Crear nueva escena vacía
    pub fn create_new_scene(&mut self, name: &str) {
        log::info!("Creando nueva escena: {}", name);
        
        let scene = Scene {
            id: Uuid::new_v4(),
            name: name.to_string(),
            metadata: Default::default(),
            nodes: Vec::new(),
            animations: Vec::new(),
            physics: Default::default(),
            camera: Default::default(),
            assets: Default::default(),
        };
        
        self.current_scene = Some(scene);
        self.scene_path = None;
        self.modified = true;
        self.undo_stack.clear();
        self.redo_stack.clear();
    }
    
    /// Abrir escena desde archivo TOML
    pub fn open_scene(&mut self, path: PathBuf) -> Result<(), anyhow::Error> {
        log::info!("Abriendo escena: {:?}", path);
        
        // Preguntar guardar si hay modificaciones
        if self.modified {
            // TODO: Implementar diálogo de guardar
            log::warn!("Hay cambios sin guardar");
        }
        
        // Leer archivo
        let content = std::fs::read_to_string(&path)?;
        
        // Parsear TOML
        let scene: Scene = toml::from_str(&content)?;
        
        // Liberar escena anterior
        if let Some(_old_scene) = self.current_scene.take() {
            // TODO: Liberar recursos de la escena anterior
            log::debug!("Escena anterior liberada");
        }
        
        // Asignar nueva escena
        self.current_scene = Some(scene);
        self.scene_path = Some(path);
        self.modified = false;
        self.undo_stack.clear();
        self.redo_stack.clear();
        
        log::info!("Escena cargada exitosamente");
        
        Ok(())
    }
    
    /// Guardar escena actual
    pub fn save_scene(&mut self, path: &PathBuf) -> Result<(), anyhow::Error> {
        if let Some(scene) = &self.current_scene {
            log::info!("Guardando escena: {:?}", path);
            
            // Serializar a TOML
            let content = toml::to_string_pretty(scene)?;
            
            // Escribir archivo
            std::fs::write(path, content)?;
            
            self.modified = false;
            self.scene_path = Some(path.clone());
            
            log::info!("Escena guardada exitosamente");
            
            Ok(())
        } else {
            Err(anyhow::anyhow!("No hay escena cargada"))
        }
    }
    
    /// Cerrar escena actual
    pub fn close_scene(&mut self) {
        log::info!("Cerrando escena actual");
        
        self.current_scene = None;
        self.scene_path = None;
        self.modified = false;
        self.undo_stack.clear();
        self.redo_stack.clear();
    }
    
    /// Marcar escena como modificada
    pub fn mark_modified(&mut self) {
        self.modified = true;
    }
    
    /// Obtener escena actual (inmutable)
    pub fn get_scene(&self) -> Option<&Scene> {
        self.current_scene.as_ref()
    }
    
    /// Obtener escena actual (mutable)
    pub fn get_scene_mut(&mut self) -> Option<&mut Scene> {
        self.current_scene.as_mut()
    }
}

impl Default for SceneManager {
    fn default() -> Self {
        Self::new()
    }
}
