// src/scripting/behavior.rs - Sistema de comportamientos

use std::collections::HashMap;
use std::boxed::Box;
use super::{Node, PropertyValue};

/// Trait para comportamientos
pub trait Behavior: Send + Sync {
    /// Actualizar comportamiento
    fn tick(&mut self, node: &mut Node, delta: f32, time: f64);
    
    /// Evento recibido
    fn on_event(&mut self, _event: &str, _params: &[(&str, PropertyValue)]) {
        // Implementación por defecto: no hacer nada
    }
    
    /// Al spawnear
    fn on_spawn(&mut self, _node: &mut Node) {}
    
    /// Al destruir
    fn on_destroy(&mut self, _node: &mut Node) {}
}

/// Comportamiento de oscilación
#[derive(Debug, Clone)]
pub struct OscillateBehavior {
    pub axis: String,
    pub speed: f32,
    pub amplitude: f32,
    pub offset: f32,
}

impl Default for OscillateBehavior {
    fn default() -> Self {
        OscillateBehavior {
            axis: "y".to_string(),
            speed: 1.0,
            amplitude: 1.0,
            offset: 0.0,
        }
    }
}

impl Behavior for OscillateBehavior {
    fn tick(&mut self, node: &mut Node, _delta: f32, time: f64) {
        if let Some(transform) = node.get_component_mut::<super::TransformComponent>() {
            let value = (time * self.speed as f64).sin() as f32 * self.amplitude + self.offset;
            
            match self.axis.as_str() {
                "x" => transform.position[0] = value,
                "y" => transform.position[1] = value,
                "z" => transform.position[2] = value,
                _ => {}
            }
        }
    }
}

/// Comportamiento de patrulla
#[derive(Debug, Clone)]
pub struct PatrolBehavior {
    pub waypoints: Vec<[f32; 3]>,
    pub speed: f32,
    pub wait_time: f32,
    pub current_target: usize,
    pub state: PatrolState,
    pub wait_timer: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PatrolState {
    Moving,
    Waiting,
}

impl Default for PatrolBehavior {
    fn default() -> Self {
        PatrolBehavior {
            waypoints: vec![],
            speed: 2.0,
            wait_time: 1.0,
            current_target: 0,
            state: PatrolState::Moving,
            wait_timer: 0.0,
        }
    }
}

impl Behavior for PatrolBehavior {
    fn tick(&mut self, node: &mut Node, delta: f32, _time: f64) {
        if self.waypoints.is_empty() {
            return;
        }
        
        match self.state {
            PatrolState::Moving => {
                let target = self.waypoints[self.current_target];
                
                if let Some(transform) = node.get_component_mut::<super::TransformComponent>() {
                    let dir = [
                        target[0] - transform.position[0],
                        target[1] - transform.position[1],
                        target[2] - transform.position[2],
                    ];
                    
                    let distance = (dir[0] * dir[0] + dir[1] * dir[1] + dir[2] * dir[2]).sqrt();
                    
                    if distance < 0.1 {
                        self.state = PatrolState::Waiting;
                        self.wait_timer = self.wait_time;
                        self.current_target = (self.current_target + 1) % self.waypoints.len();
                    } else {
                        let scale = (self.speed * delta) / distance;
                        transform.position[0] += dir[0] * scale;
                        transform.position[1] += dir[1] * scale;
                        transform.position[2] += dir[2] * scale;
                    }
                }
            }
            
            PatrolState::Waiting => {
                self.wait_timer -= delta;
                if self.wait_timer <= 0.0 {
                    self.state = PatrolState::Moving;
                }
            }
        }
    }
}

/// Registro de comportamientos disponibles
pub type BehaviorRegistry = HashMap<String, Box<dyn Fn() -> Box<dyn Behavior>>>;

fn create_oscillate_behavior() -> Box<dyn Behavior> {
    Box::new(OscillateBehavior::default())
}

fn create_patrol_behavior() -> Box<dyn Behavior> {
    Box::new(PatrolBehavior::default())
}

/// Obtener registro global de comportamientos
pub fn get_behavior_registry() -> BehaviorRegistry {
    let mut registry = HashMap::new();

    registry.insert(
        "oscillate".to_string(),
        Box::new(create_oscillate_behavior)
    );

    registry.insert(
        "patrol".to_string(),
        Box::new(create_patrol_behavior)
    );

    registry
}
