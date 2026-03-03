// src/ecs/mod.rs - Entity-Component-System

mod world;
mod entity;
mod system;

pub use world::EcsWorld;
pub use entity::Entity;
pub use system::System;

/// Componente base (marker trait)
pub trait Component: Send + Sync + 'static {}

/// Sistema del ECS
pub trait SystemTrait: Send + Sync {
    fn update(&mut self, world: &mut EcsWorld, delta: f32);
}
