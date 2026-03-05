// src/scripting/scene.rs - Estructura de escena

use uuid::Uuid;
use super::{SceneMetadata, PhysicsSettings, SceneAssets, Node};
use crate::animation::Animation;

/// Escena del juego
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Scene {
    /// ID único de la escena
    pub id: Uuid,
    
    /// Nombre de la escena
    pub name: String,
    
    /// Metadata
    pub metadata: SceneMetadata,
    
    /// Nodos de la escena
    pub nodes: Vec<Node>,
    
    /// Animaciones
    #[serde(default)]
    pub animations: Vec<Animation>,
    
    /// Configuración de físicas
    #[serde(default)]
    pub physics: PhysicsSettings,
    
    /// Cámara por defecto
    #[serde(default)]
    pub camera: CameraData,
    
    /// Assets de la escena
    #[serde(default)]
    pub assets: SceneAssets,
}

/// Datos de cámara
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CameraData {
    pub position: [f32; 3],
    pub target: [f32; 3],
    pub up: [f32; 3],
    pub fov: f32,
    pub near: f32,
    pub far: f32,
}

impl Default for CameraData {
    fn default() -> Self {
        CameraData {
            position: [5.0, 5.0, 5.0],
            target: [0.0, 0.0, 0.0],
            up: [0.0, 1.0, 0.0],
            fov: 60.0,
            near: 0.1,
            far: 1000.0,
        }
    }
}

impl Scene {
    /// Buscar un nodo por ID
    pub fn find_node(&self, id: Uuid) -> Option<&Node> {
        self.nodes.iter().find(|n| n.id == id)
    }
    
    /// Buscar un nodo por ID (mutable)
    pub fn find_node_mut(&mut self, id: Uuid) -> Option<&mut Node> {
        self.nodes.iter_mut().find(|n| n.id == id)
    }
    
    /// Añadir un nodo
    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }
    
    /// Eliminar un nodo
    pub fn remove_node(&mut self, id: Uuid) -> Option<Node> {
        if let Some(pos) = self.nodes.iter().position(|n| n.id == id) {
            Some(self.nodes.remove(pos))
        } else {
            None
        }
    }
}
