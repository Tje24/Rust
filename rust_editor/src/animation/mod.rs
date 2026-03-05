// src/animation/mod.rs - Módulo de animación

mod animation;
mod timeline;

pub use animation::{Animation, LoopMode};
pub use timeline::Timeline;

/// Track de animación
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AnimationTrack {
    pub id: uuid::Uuid,
    pub name: String,
    pub node_id: uuid::Uuid,
    pub property: TrackProperty,
    pub keyframes: Vec<Keyframe>,
}

/// Propiedad a animar
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum TrackProperty {
    Position,
    Rotation,
    Scale,
    Color,
    Visibility,
}

/// Keyframe de animación
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Keyframe {
    pub id: uuid::Uuid,
    pub time: f32,
    pub value: KeyframeValue,
}

/// Valor de keyframe
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum KeyframeValue {
    Vec3([f32; 3]),
    Vec4([f32; 4]),
    Float(f32),
    Boolean(bool),
}
