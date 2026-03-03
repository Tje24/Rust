// src/animation/timeline.rs - Timeline de animación

use super::Animation;

/// Timeline para edición de animaciones
pub struct Timeline {
    pub current_time: f32,
    pub is_playing: bool,
    pub zoom: f32,
    pub selected_track: Option<uuid::Uuid>,
    pub selected_keyframe: Option<uuid::Uuid>,
}

impl Timeline {
    pub fn new() -> Self {
        Timeline {
            current_time: 0.0,
            is_playing: false,
            zoom: 1.0,
            selected_track: None,
            selected_keyframe: None,
        }
    }
    
    pub fn play(&mut self) {
        self.is_playing = true;
    }
    
    pub fn pause(&mut self) {
        self.is_playing = false;
    }
    
    pub fn stop(&mut self) {
        self.is_playing = false;
        self.current_time = 0.0;
    }
    
    pub fn update(&mut self, delta: f32, animation: &Animation) {
        if self.is_playing {
            self.current_time += delta;
            
            if self.current_time >= animation.length {
                match animation.loop_mode {
                    super::LoopMode::Loop => {
                        self.current_time = 0.0;
                    }
                    super::LoopMode::Once => {
                        self.current_time = animation.length;
                        self.is_playing = false;
                    }
                    super::LoopMode::PingPong => {
                        // TODO: Implementar ping pong
                    }
                }
            }
        }
    }
}

impl Default for Timeline {
    fn default() -> Self {
        Self::new()
    }
}
