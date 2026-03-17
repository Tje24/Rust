// src/core/editor_state.rs - Estado del editor

use super::{EditorMode, ViewportMode, RenderMode};
use uuid::Uuid;

/// Estado global del editor
#[derive(Debug, Clone)]
pub struct EditorState {
    /// Modo actual del editor
    pub editor_mode: EditorMode,
    
    /// Modo del viewport
    pub viewport_mode: ViewportMode,
    
    /// Modo de renderizado
    pub render_mode: RenderMode,
    
    /// Nodo seleccionado
    pub selected_node: Option<Uuid>,
    
    /// Nodos en selección múltiple
    pub selected_nodes: Vec<Uuid>,
    
    /// Nodo bajo el hover
    pub hovered_node: Option<Uuid>,
    
    /// Cámara del viewport
    pub viewport_camera: ViewportCamera,
    
    /// Configuración de grid
    pub grid_settings: GridSettings,
    
    /// Configuración de snapping
    pub snap_settings: SnapSettings,
}

/// Configuración de cámara del viewport
#[derive(Debug, Clone)]
pub struct ViewportCamera {
    pub position: [f32; 3],
    pub target: [f32; 3],
    pub up: [f32; 3],
    pub fov: f32,
    pub zoom: f32,
}

/// Configuración del grid
#[derive(Debug, Clone)]
pub struct GridSettings {
    pub enabled: bool,
    pub size: f32,
    pub subdivisions: u32,
    pub color: [u8; 4],
}

/// Configuración de snapping
#[derive(Debug, Clone)]
pub struct SnapSettings {
    pub enabled: bool,
    pub snap_to_grid: bool,
    pub snap_to_vertex: bool,
    pub snap_distance: f32,
}

impl Default for EditorState {
    fn default() -> Self {
        EditorState {
            editor_mode: EditorMode::Edit,
            viewport_mode: ViewportMode::Mode3D,
            render_mode: RenderMode::Wgpu,
            selected_node: None,
            selected_nodes: Vec::new(),
            hovered_node: None,
            viewport_camera: ViewportCamera::default(),
            grid_settings: GridSettings::default(),
            snap_settings: SnapSettings::default(),
        }
    }
}

impl Default for ViewportCamera {
    fn default() -> Self {
        ViewportCamera {
            position: [5.0, 5.0, 5.0],
            target: [0.0, 0.0, 0.0],
            up: [0.0, 1.0, 0.0],
            fov: 60.0,
            zoom: 1.0,
        }
    }
}

impl Default for GridSettings {
    fn default() -> Self {
        GridSettings {
            enabled: true,
            size: 10.0,
            subdivisions: 10,
            color: [100, 100, 100, 255],
        }
    }
}

impl Default for SnapSettings {
    fn default() -> Self {
        SnapSettings {
            enabled: true,
            snap_to_grid: true,
            snap_to_vertex: false,
            snap_distance: 0.1,
        }
    }
}
