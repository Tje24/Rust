// src/ecs/world.rs - Mundo ECS

use super::Entity;
use uuid::Uuid;
use std::collections::HashMap;

/// Mundo ECS
pub struct EcsWorld {
    pub entities: HashMap<Uuid, Entity>,
    pub next_id: u64,
}

impl EcsWorld {
    pub fn new() -> Self {
        EcsWorld {
            entities: HashMap::new(),
            next_id: 0,
        }
    }
    
    pub fn create_entity(&mut self) -> Entity {
        let id = Uuid::new_v4();
        let entity = Entity::new(id);
        self.entities.insert(id, Entity::new(id));
        self.next_id += 1;
        entity
    }
    
    pub fn destroy_entity(&mut self, id: Uuid) {
        self.entities.remove(&id);
    }
    
    pub fn get_entity(&self, id: Uuid) -> Option<&Entity> {
        self.entities.get(&id)
    }
    
    pub fn get_entity_mut(&mut self, id: Uuid) -> Option<&mut Entity> {
        self.entities.get_mut(&id)
    }
    
    pub fn update(&mut self, delta: f32) {
        // Actualizar todas las entidades
        for entity in self.entities.values_mut() {
            entity.update(delta);
        }
    }
}

impl Default for EcsWorld {
    fn default() -> Self {
        Self::new()
    }
}
