// src/core/mod.rs - Módulo core del editor

mod editor_app;
mod scene_manager;
mod editor_state;

pub use editor_app::EditorApp;
pub use scene_manager::SceneManager;
pub use editor_state::EditorState;

/// Modos del editor
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditorMode {
    /// Modo edición (viewport editable)
    Edit,
    /// Modo juego (preview en tiempo real)
    Play,
}

/// Modos del viewport
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewportMode {
    /// Vista 2D
    Mode2D,
    /// Vista 3D
    Mode3D,
    /// Vista de scripting
    Script,
}

/// Modos de renderizado
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderMode {
    /// wgpu (máximo rendimiento)
    Bestia,
    /// raylib (máxima compatibilidad)
    Dinamico,
}

impl Default for EditorMode {
    fn default() -> Self {
        EditorMode::Edit
    }
}

impl Default for ViewportMode {
    fn default() -> Self {
        ViewportMode::Mode3D
    }
}

impl Default for RenderMode {
    fn default() -> Self {
        RenderMode::Dinamico // Empezar con el más compatible
    }
}
