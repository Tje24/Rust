// src/scripting/components.rs - Componentes del ECS

use glam::{Vec3, Quat};

/// Componentes disponibles
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum Component {
    Transform(TransformComponent),
    Mesh(MeshComponent),
    Sprite(SpriteComponent),
    Camera(CameraComponent),
    Light(LightComponent),
    RigidBody(RigidBodyComponent),
    Collider(ColliderComponent),
    Behavior(BehaviorComponent),
    AudioSource(AudioSourceComponent),
}

/// Componente de transformación
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TransformComponent {
    pub position: [f32; 3],
    pub rotation: [f32; 4], // Quaternion
    pub scale: [f32; 3],
}

impl Default for TransformComponent {
    fn default() -> Self {
        TransformComponent {
            position: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0, 1.0],
            scale: [1.0, 1.0, 1.0],
        }
    }
}

impl TransformComponent {
    pub fn position(&self) -> Vec3 {
        Vec3::from(self.position)
    }
    
    pub fn rotation(&self) -> Quat {
        Quat::from_array(self.rotation)
    }
    
    pub fn scale(&self) -> Vec3 {
        Vec3::from(self.scale)
    }
}

/// Componente de malla
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MeshComponent {
    pub mesh_id: String,
    pub material_id: Option<String>,
}

/// Componente de sprite (2D)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SpriteComponent {
    pub texture_id: String,
    pub color: [u8; 4],
    pub flip_x: bool,
    pub flip_y: bool,
}

/// Componente de cámara
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CameraComponent {
    pub fov: f32,
    pub near: f32,
    pub far: f32,
    pub is_active: bool,
}

/// Componente de luz
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LightComponent {
    pub light_type: LightType,
    pub color: [f32; 3],
    pub intensity: f32,
    pub range: f32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum LightType {
    Directional,
    Point,
    Spot,
}

/// Componente de cuerpo rígido (físicas)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RigidBodyComponent {
    pub mass: f32,
    pub linear_damping: f32,
    pub angular_damping: f32,
    pub is_kinematic: bool,
}

/// Componente de colisor
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ColliderComponent {
    pub shape: ColliderShape,
    pub is_trigger: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ColliderShape {
    Box { size: [f32; 3] },
    Sphere { radius: f32 },
    Capsule { radius: f32, height: f32 },
}

/// Componente de comportamiento (scripting)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BehaviorComponent {
    pub behavior_id: String,
    pub properties: std::collections::HashMap<String, PropertyValue>,
    pub enabled: bool,
}

/// Componente de fuente de audio
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AudioSourceComponent {
    pub clip_id: String,
    pub volume: f32,
    pub loop_sound: bool,
    pub is_playing: bool,
}

/// Valores de propiedad para comportamientos
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum PropertyValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Vec2([f32; 2]),
    Vec3([f32; 3]),
    Vec4([f32; 4]),
    Color([u8; 4]),
}

// Implementar traits para conversión de componentes
impl super::AsComponent for TransformComponent {
    fn as_ref(component: &Component) -> Option<&Self> {
        if let Component::Transform(t) = component {
            Some(t)
        } else {
            None
        }
    }
    
    fn as_mut(component: &mut Component) -> Option<&mut Self> {
        if let Component::Transform(t) = component {
            Some(t)
        } else {
            None
        }
    }
}

impl super::AsComponent for MeshComponent {
    fn as_ref(component: &Component) -> Option<&Self> {
        if let Component::Mesh(m) = component {
            Some(m)
        } else {
            None
        }
    }
    
    fn as_mut(component: &mut Component) -> Option<&mut Self> {
        if let Component::Mesh(m) = component {
            Some(m)
        } else {
            None
        }
    }
}
