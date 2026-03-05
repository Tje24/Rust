// src/scripting/node.rs - Estructura de nodo

use uuid::Uuid;
use super::Component;

/// Nodo de la escena (Entity en ECS)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Node {
    /// ID único
    pub id: Uuid,
    
    /// Nombre del nodo
    pub name: String,
    
    /// ID del padre (si existe)
    #[serde(default)]
    pub parent: Option<Uuid>,
    
    /// IDs de los hijos
    #[serde(default)]
    pub children: Vec<Uuid>,
    
    /// Componentes del nodo
    #[serde(default)]
    pub components: Vec<Component>,
    
    /// Tags para búsqueda
    #[serde(default)]
    pub tags: Vec<String>,
    
    /// ¿Es visible?
    #[serde(default = "default_true")]
    pub visible: bool,
    
    /// ¿Está activo?
    #[serde(default = "default_true")]
    pub enabled: bool,
}

fn default_true() -> bool {
    true
}

impl Node {
    /// Crear un nuevo nodo
    pub fn new(name: &str) -> Self {
        Node {
            id: Uuid::new_v4(),
            name: name.to_string(),
            parent: None,
            children: Vec::new(),
            components: Vec::new(),
            tags: Vec::new(),
            visible: true,
            enabled: true,
        }
    }
    
    /// Añadir un componente
    pub fn add_component(&mut self, component: Component) {
        self.components.push(component);
    }
    
    /// Obtener un componente
    pub fn get_component<T: AsComponent>(&self) -> Option<&T> {
        self.components.iter().find_map(|c| c.as_ref::<T>())
    }

    /// Obtener un componente (mutable)
    pub fn get_component_mut<T: AsComponent>(&mut self) -> Option<&mut T> {
        self.components.iter_mut().find_map(|c| c.as_mut::<T>())
    }

    /// Verificar si tiene un componente
    pub fn has_component<T: AsComponent>(&self) -> bool {
        self.components.iter().any(|c| c.is::<T>())
    }
    
    /// Verificar si tiene un tag
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.contains(&tag.to_string())
    }
    
    /// Obtener malla (si existe)
    pub fn get_mesh(&self) -> Option<&super::MeshComponent> {
        self.get_component()
    }
}

/// Trait para convertir componentes
pub trait AsComponent: 'static {
    fn as_ref(component: &Component) -> Option<&Self>;
    fn as_mut(component: &mut Component) -> Option<&mut Self>;
}
