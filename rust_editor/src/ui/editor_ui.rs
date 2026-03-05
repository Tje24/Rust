// src/ui/editor_ui.rs - UI principal del editor

use std::string::String;

use super::{PanelState, BottomTab};
use crate::core::EditorApp;

/// UI del editor con egui
pub struct EditorUI {
    /// Panel izquierdo (archivos)
    pub left_panel: PanelState,
    
    /// Panel derecho (inspector)
    pub right_panel: PanelState,
    
    /// Panel inferior (animación, tilemap, etc)
    pub bottom_panel: PanelState,
    
    /// Pestaña activa en panel inferior
    pub bottom_tab: BottomTab,
    
    /// Query de búsqueda
    pub search_query: String,
}

impl EditorUI {
    /// Crear nueva UI
    pub fn new() -> Self {
        EditorUI {
            left_panel: PanelState::default(),
            right_panel: PanelState::default(),
            bottom_panel: PanelState::default(),
            bottom_tab: BottomTab::default(),
            search_query: String::new(),
        }
    }
    
    /// Actualizar UI
    pub fn update(&mut self, _delta: f32) {
        // Actualizar lógica de UI si es necesario
    }
    
    /// Dibujar panel de archivos
    pub fn draw_files_panel(&mut self, ui: &mut egui::Ui, _mode: &mut crate::core::EditorMode, _viewport_mode: &mut crate::core::ViewportMode, _project_path: &Option<std::path::PathBuf>) {
        ui.heading("📁 Archivos");
        ui.separator();

        ui.label("No hay proyecto abierto");

        ui.add_space(10.0);

        ui.horizontal(|ui| {
            if ui.button("➕ Nueva escena").clicked() {
                log::info!("Crear nueva escena");
            }

            if ui.button("📂 Abrir").clicked() {
                log::info!("Abrir escena");
            }
        });
    }
    
    /// Dibujar inspector
    pub fn draw_inspector(&mut self, ui: &mut egui::Ui, _app: &mut EditorApp) {
        ui.heading("🔍 Inspector");
        ui.separator();
        
        ui.label("No hay nodo seleccionado");
        
        ui.add_space(10.0);

        ui.collapsing("Transform", |ui| {
            ui.horizontal(|ui| {
                ui.label("Position:");
                let mut pos_x = 0.0f32;
                let mut pos_y = 0.0f32;
                let mut pos_z = 0.0f32;
                ui.add(egui::DragValue::new(&mut pos_x).speed(0.1));
                ui.add(egui::DragValue::new(&mut pos_y).speed(0.1));
                ui.add(egui::DragValue::new(&mut pos_z).speed(0.1));
            });
        });
    }
    
    /// Dibujar viewport
    pub fn draw_viewport(&mut self, ui: &mut egui::Ui, _renderer: &mut Option<Box<dyn crate::render::RenderBackend>>, _scene_manager: &mut crate::core::SceneManager) {
        // Barra de herramientas superior
        ui.horizontal(|ui| {
            ui.label("Viewport - Próximamente: renderizado 3D");
        });

        ui.separator();

        // Área del viewport
        let viewport_rect = ui.available_rect_before_wrap();

        // Fondo del viewport
        ui.painter().rect_filled(
            viewport_rect,
            0.0,
            egui::Color32::from_rgb(30, 30, 30),
        );

        // Texto de placeholder
        ui.painter().text(
            viewport_rect.center(),
            egui::Align2::CENTER_CENTER,
            "Viewport - Próximamente: renderizado 3D",
            egui::FontId::proportional(16.0),
            egui::Color32::GRAY,
        );

        // FPS counter - use egui's fps method from context
        let fps = ui.ctx().fps();
        ui.label(format!("FPS: {:.0}", fps));
    }

    /// Dibujar panel inferior
    pub fn draw_bottom_panel(&mut self, ui: &mut egui::Ui, _mode: &crate::core::EditorMode) {
        // Pestañas
        ui.horizontal(|ui| {
            ui.selectable_value(
                &mut self.bottom_tab,
                BottomTab::Animation,
                "🎬 Animación"
            );
            ui.selectable_value(
                &mut self.bottom_tab,
                BottomTab::Tilemap,
                "🗺️ Tilemap"
            );
            ui.selectable_value(
                &mut self.bottom_tab,
                BottomTab::Debug,
                "🐞 Debug"
            );
            ui.selectable_value(
                &mut self.bottom_tab,
                BottomTab::Output,
                "📋 Output"
            );
        });
        
        ui.separator();
        
        // Contenido según pestaña
        match self.bottom_tab {
            BottomTab::Animation => {
                ui.label("Timeline de animación (próximamente)");
            }
            BottomTab::Tilemap => {
                ui.label("Editor de Tilemap (próximamente)");
            }
            BottomTab::Debug => {
                ui.label("Consola de debug");
                ui.label("Log messages aparecerán aquí");
            }
            BottomTab::Output => {
                ui.label("Salida de compilación");
            }
        }
    }
}

impl Default for EditorUI {
    fn default() -> Self {
        Self::new()
    }
}
