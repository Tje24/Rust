// src/render/render_backend.rs - Trait común para backends de renderizado

use super::PerformanceMetrics;
use crate::scripting::Scene;
use glam::Vec3;

/// Tipo de backend
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackendType {
    /// wgpu (Vulkan/Metal/DirectX)
    Wgpu,
}

/// Error de renderizado
#[derive(Debug, thiserror::Error)]
pub enum RenderError {
    #[error("Error de inicialización: {0}")]
    Initialization(String),
    
    #[error("Error de dispositivo: {0}")]
    Device(String),
    
    #[error("Error de recurso: {0}")]
    Resource(String),
    
    #[error("Error de shader: {0}")]
    Shader(String),
}

/// Handle para mallas
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MeshHandle(pub u32);

/// Handle para texturas
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TextureHandle(pub u32);

/// Datos de malla
#[derive(Debug, Clone)]
pub struct MeshData {
    pub vertices: Vec<f32>,
    pub indices: Vec<u32>,
    pub stride: u32,
}

/// Datos de textura
#[derive(Debug, Clone)]
pub struct TextureData {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub channels: u8,
}

/// Cámara para renderizado
#[derive(Debug, Clone)]
pub struct Camera {
    pub position: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub fov: f32,
    pub near: f32,
    pub far: f32,
    pub aspect: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            position: Vec3::new(5.0, 5.0, 5.0),
            target: Vec3::ZERO,
            up: Vec3::Y,
            fov: 60.0_f32.to_radians(),
            near: 0.1,
            far: 1000.0,
            aspect: 16.0 / 9.0,
        }
    }
}

/// Trait común para todos los backends de renderizado
pub trait RenderBackend: Send + Sync {
    /// Inicializar el backend
    fn new() -> Result<Self, RenderError>
    where
        Self: Sized;
    
    /// Comenzar un nuevo frame
    fn begin_frame(&mut self);
    
    /// Renderizar la escena
    fn render_scene(&mut self, scene: &Scene);
    
    /// Terminar el frame
    fn end_frame(&mut self);
    
    /// Cargar una malla
    fn load_mesh(&mut self, mesh_data: &MeshData) -> Result<MeshHandle, RenderError>;
    
    /// Cargar una textura
    fn load_texture(&mut self, texture_data: &TextureData) -> Result<TextureHandle, RenderError>;
    
    /// Descargar una malla
    fn unload_mesh(&mut self, handle: MeshHandle);
    
    /// Descargar una textura
    fn unload_texture(&mut self, handle: TextureHandle);
    
    /// Establecer cámara
    fn set_camera(&mut self, camera: &Camera);
    
    /// Obtener cámara actual
    fn get_camera(&self) -> Camera;
    
    /// Obtener métricas de rendimiento
    fn get_metrics(&self) -> PerformanceMetrics;
    
    /// Obtener tipo de backend
    fn get_backend_type(&self) -> BackendType;
    
    /// Validar si el frame es válido
    fn validate_frame(&self) -> bool {
        true
    }
}
