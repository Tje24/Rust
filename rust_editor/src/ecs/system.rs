// src/ecs/system.rs - Sistema del ECS

use super::EcsWorld;

/// Sistema base
pub trait System: Send + Sync {
    fn update(&mut self, world: &mut EcsWorld, delta: f32);
}

/// Sistema de movimiento
pub struct MoveSystem;

impl System for MoveSystem {
    fn update(&mut self, _world: &mut EcsWorld, _delta: f32) {
        // TODO: Implementar sistema de movimiento
    }
}

/// Sistema de físicas
pub struct PhysicsSystem;

impl System for PhysicsSystem {
    fn update(&mut self, _world: &mut EcsWorld, _delta: f32) {
        // TODO: Implementar sistema de físicas
    }
}

/// Sistema de renderizado
pub struct RenderSystem;

impl System for RenderSystem {
    fn update(&mut self, _world: &mut EcsWorld, _delta: f32) {
        // TODO: Implementar sistema de renderizado
    }
}
