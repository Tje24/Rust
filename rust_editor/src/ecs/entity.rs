// src/ecs/entity.rs - Entidad del ECS

use uuid::Uuid;
use std::collections::HashMap;
use std::boxed::Box;

/// Entidad
#[derive(Debug)]
pub struct Entity {
    pub id: Uuid,
    pub components: HashMap<String, Box<dyn std::any::Any + Send + Sync>>,
    pub active: bool,
}

impl Entity {
    pub fn new(id: Uuid) -> Self {
        Entity {
            id,
            components: HashMap::new(),
            active: true,
        }
    }
    
    pub fn add_component<T: 'static + Send + Sync>(&mut self, name: &str, component: T) {
        self.components.insert(name.to_string(), Box::new(component));
    }
    
    pub fn get_component<T: 'static>(&self, name: &str) -> Option<&T> {
        self.components.get(name).and_then(|c| c.downcast_ref())
    }
    
    pub fn get_component_mut<T: 'static>(&mut self, name: &str) -> Option<&mut T> {
        self.components.get_mut(name).and_then(|c| c.downcast_mut())
    }
    
    pub fn remove_component(&mut self, name: &str) {
        self.components.remove(name);
    }
    
    pub fn has_component(&self, name: &str) -> bool {
        self.components.contains_key(name)
    }
    
    pub fn update(&mut self, _delta: f32) {
        // Actualizar componentes si es necesario
    }
}
