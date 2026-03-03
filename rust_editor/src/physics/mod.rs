// src/physics/mod.rs - Módulo de físicas

mod physics_world;

pub use physics_world::PhysicsWorld;

/// Tipo de colisor
#[derive(Debug, Clone)]
pub enum ColliderType {
    Box { size: glam::Vec3 },
    Sphere { radius: f32 },
    Capsule { radius: f32, height: f32 },
    Mesh { vertices: Vec<glam::Vec3> },
}

/// Información de colisión
#[derive(Debug, Clone)]
pub struct CollisionInfo {
    pub entity_a: uuid::Uuid,
    pub entity_b: uuid::Uuid,
    pub normal: glam::Vec3,
    pub depth: f32,
    pub contact_point: glam::Vec3,
}
