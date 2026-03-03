// src/ui/mod.rs - Módulo de UI con egui

mod editor_ui;
mod panels;

pub use editor_ui::EditorUI;
pub use panels::*;

/// Estado de un panel
#[derive(Debug, Clone)]
pub struct PanelState {
    /// ¿Está expandido?
    pub expanded: bool,
    /// Ancho del panel
    pub width: f32,
    /// Alto del panel
    pub height: f32,
    /// ¿Es visible?
    pub visible: bool,
}

impl Default for PanelState {
    fn default() -> Self {
        PanelState {
            expanded: true,
            width: 250.0,
            height: 200.0,
            visible: true,
        }
    }
}

/// Pestañas del panel inferior
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BottomTab {
    Animation,
    Tilemap,
    Debug,
    Output,
}

impl Default for BottomTab {
    fn default() -> Self {
        BottomTab::Animation
    }
}
