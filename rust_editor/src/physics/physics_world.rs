// src/physics/physics_world.rs - Mundo de físicas

use super::CollisionInfo;

/// Mundo de físicas
pub struct PhysicsWorld {
    pub gravity: glam::Vec3,
    pub substeps: u32,
}

impl PhysicsWorld {
    pub fn new() -> Self {
        PhysicsWorld {
            gravity: glam::Vec3::new(0.0, -9.8, 0.0),
            substeps: 4,
        }
    }
    
    pub fn update(&mut self, delta: f32) {
        // TODO: Implementar simulación de físicas
        let _step = delta / self.substeps as f32;
    }
    
    pub fn check_collisions(&self) -> Vec<CollisionInfo> {
        // TODO: Implementar detección de colisiones
        Vec::new()
    }
}

impl Default for PhysicsWorld {
    fn default() -> Self {
        Self::new()
    }
}
