// src/animation/animation.rs - Estructura de animación

use uuid::Uuid;
use super::{AnimationTrack, KeyframeValue};

/// Modo de loop de animación
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum LoopMode {
    Once,
    Loop,
    PingPong,
}

/// Animación
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Animation {
    pub id: Uuid,
    pub name: String,
    pub length: f32,
    pub fps: u32,
    pub loop_mode: LoopMode,
    pub tracks: Vec<AnimationTrack>,
}

impl Default for Animation {
    fn default() -> Self {
        Animation {
            id: Uuid::new_v4(),
            name: "New Animation".to_string(),
            length: 5.0,
            fps: 30,
            loop_mode: LoopMode::Loop,
            tracks: Vec::new(),
        }
    }
}

impl Animation {
    /// Obtener duración de la animación
    pub fn duration(&self) -> f32 {
        self.length
    }
    
    /// Evaluar animación en un tiempo dado
    pub fn evaluate(&self, time: f32) -> Vec<(Uuid, KeyframeValue)> {
        // TODO: Implementar evaluación de tracks
        Vec::new()
    }
}
