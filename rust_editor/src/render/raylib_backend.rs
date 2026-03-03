// src/render/raylib_backend.rs - Backend usando raylib (OpenGL ES)

use super::{
    RenderBackend, RenderError, BackendType, PerformanceMetrics,
    MeshHandle, TextureHandle, MeshData, TextureData, Camera,
};
use crate::scripting::Scene;
use glam::Vec3;

/// Backend de renderizado usando raylib
/// Máxima compatibilidad con dispositivos gama baja/media
pub struct RaylibBackend {
    /// Cámara actual
    camera: Camera,
    
    /// Mallas cargadas
    meshes: std::collections::HashMap<MeshHandle, RaylibMesh>,
    
    /// Texturas cargadas
    textures: std::collections::HashMap<TextureHandle, RaylibTexture>,
    
    /// Métricas de rendimiento
    metrics: PerformanceMetrics,
    
    /// Contador de draw calls
    draw_calls: u32,
    
    /// Contador de polígonos
    polygons: u32,
}

/// Malla en formato raylib
pub struct RaylibMesh {
    pub vertex_count: u32,
    pub indices: Vec<u32>,
    pub vertices: Vec<f32>,
}

/// Textura en formato raylib
pub struct RaylibTexture {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

impl RaylibBackend {
    /// Crear nuevo backend de raylib
    pub fn new() -> Self {
        log::info!("Inicializando RaylibBackend...");
        
        RaylibBackend {
            camera: Camera::default(),
            meshes: std::collections::HashMap::new(),
            textures: std::collections::HashMap::new(),
            metrics: PerformanceMetrics::default(),
            draw_calls: 0,
            polygons: 0,
        }
    }
    
    /// Actualizar métricas
    fn update_metrics(&mut self) {
        // En implementación real, obtener del sistema
        self.metrics.fps = 60;
        self.metrics.frame_time_ms = 16.67;
        self.metrics.draw_calls = self.draw_calls;
        self.metrics.polygons = self.polygons;
    }
}

impl RenderBackend for RaylibBackend {
    fn new() -> Result<Self, RenderError> {
        Ok(Self::new())
    }
    
    fn begin_frame(&mut self) {
        self.draw_calls = 0;
        self.polygons = 0;
    }
    
    fn render_scene(&mut self, scene: &Scene) {
        // En implementación real:
        // 1. Limpiar buffer
        // 2. Iterar sobre nodos de la escena
        // 3. Renderizar cada nodo con su malla y textura
        
        for node in &scene.nodes {
            // TODO: Renderizar nodo
            self.draw_calls += 1;
            
            if let Some(_mesh) = node.get_mesh() {
                self.polygons += 100; // Estimado
            }
        }
        
        self.update_metrics();
    }
    
    fn end_frame(&mut self) {
        // En implementación real, swap buffers
    }
    
    fn load_mesh(&mut self, mesh_data: &MeshData) -> Result<MeshHandle, RenderError> {
        let handle = MeshHandle(self.meshes.len() as u32);
        
        let mesh = RaylibMesh {
            vertex_count: (mesh_data.vertices.len() / mesh_data.stride as usize) as u32,
            indices: mesh_data.indices.clone(),
            vertices: mesh_data.vertices.clone(),
        };
        
        self.meshes.insert(handle, mesh);
        
        log::debug!("Malla cargada: {:?}", handle);
        
        Ok(handle)
    }
    
    fn load_texture(&mut self, texture_data: &TextureData) -> Result<TextureHandle, RenderError> {
        let handle = TextureHandle(self.textures.len() as u32);
        
        let texture = RaylibTexture {
            width: texture_data.width,
            height: texture_data.height,
            data: texture_data.data.clone(),
        };
        
        self.textures.insert(handle, texture);
        
        log::debug!("Textura cargada: {:?}", handle);
        
        Ok(handle)
    }
    
    fn unload_mesh(&mut self, handle: MeshHandle) {
        self.meshes.remove(&handle);
        log::debug!("Malla descargada: {:?}", handle);
    }
    
    fn unload_texture(&mut self, handle: TextureHandle) {
        self.textures.remove(&handle);
        log::debug!("Textura descargada: {:?}", handle);
    }
    
    fn set_camera(&mut self, camera: &Camera) {
        self.camera = camera.clone();
    }
    
    fn get_camera(&self) -> Camera {
        self.camera.clone()
    }
    
    fn get_metrics(&self) -> PerformanceMetrics {
        self.metrics.clone()
    }
    
    fn get_backend_type(&self) -> BackendType {
        BackendType::Raylib
    }
}
