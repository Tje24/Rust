// src/ui/panels.rs - Paneles individuales de la UI

use crate::core::EditorApp;

/// Panel de archivos (árbol de directorios)
pub struct FilesPanel {
    pub root_path: Option<std::path::PathBuf>,
}

impl FilesPanel {
    pub fn new() -> Self {
        FilesPanel { root_path: None }
    }
    
    pub fn draw(&mut self, ui: &mut egui::Ui, _app: &mut EditorApp) {
        ui.heading("📁 Archivos");
        ui.separator();
        
        if let Some(root) = &self.root_path {
            // Mostrar árbol de directorios
            self.draw_directory(ui, root);
        } else {
            ui.label("No hay proyecto abierto");
            
            if ui.button("Abrir proyecto").clicked() {
                // TODO: Abrir diálogo de archivos
                log::info!("Abrir proyecto");
            }
        }
    }
    
    fn draw_directory(&mut self, ui: &mut egui::Ui, path: &std::path::Path) {
        // Implementación futura: árbol de directorios
        ui.label(format!("📁 {:?}", path.file_name().unwrap_or_default()));
    }
}

impl Default for FilesPanel {
    fn default() -> Self {
        Self::new()
    }
}

/// Panel del inspector de propiedades
pub struct InspectorPanel {
    pub selected_node_id: Option<uuid::Uuid>,
}

impl InspectorPanel {
    pub fn new() -> Self {
        InspectorPanel { selected_node_id: None }
    }
    
    pub fn draw(&mut self, ui: &mut egui::Ui, app: &mut EditorApp) {
        ui.heading("🔍 Inspector");
        ui.separator();
        
        if let Some(scene) = app.scene_manager.get_scene() {
            if let Some(node_id) = self.selected_node_id {
                // Buscar nodo seleccionado
                if let Some(node) = scene.nodes.iter().find(|n| n.id == node_id) {
                    self.draw_node_properties(ui, node);
                } else {
                    ui.label("Nodo no encontrado");
                }
            } else {
                // Mostrar propiedades de la escena
                self.draw_scene_properties(ui, scene);
            }
        } else {
            ui.label("No hay escena cargada");
        }
    }
    
    fn draw_scene_properties(&mut self, ui: &mut egui::Ui, scene: &crate::scripting::Scene) {
        ui.label(format!("Nombre: {}", scene.name));
        
        ui.collapsing("Configuración", |ui| {
            ui.label("Propiedades de la escena");
        });
        
        ui.collapsing("Assets", |ui| {
            ui.label(format!("Meshes: {}", scene.assets.meshes.len()));
            ui.label(format!("Texturas: {}", scene.assets.textures.len()));
        });
    }
    
    fn draw_node_properties(&mut self, ui: &mut egui::Ui, _node: &crate::scripting::Node) {
        ui.collapsing("Transform", |ui| {
            ui.horizontal(|ui| {
                ui.label("Position:");
                ui.add(egui::DragValue::f32(&mut 0.0).speed(0.1));
                ui.add(egui::DragValue::f32(&mut 0.0).speed(0.1));
                ui.add(egui::DragValue::f32(&mut 0.0).speed(0.1));
            });
            
            ui.horizontal(|ui| {
                ui.label("Rotation:");
                ui.add(egui::DragValue::f32(&mut 0.0).speed(1.0));
                ui.add(egui::DragValue::f32(&mut 0.0).speed(1.0));
                ui.add(egui::DragValue::f32(&mut 0.0).speed(1.0));
            });
            
            ui.horizontal(|ui| {
                ui.label("Scale:");
                ui.add(egui::DragValue::f32(&mut 1.0).speed(0.1));
                ui.add(egui::DragValue::f32(&mut 1.0).speed(0.1));
                ui.add(egui::DragValue::f32(&mut 1.0).speed(0.1));
            });
        });
        
        ui.collapsing("Componentes", |ui| {
            ui.label("Componentes del nodo");
            
            if ui.button("➕ Añadir componente").clicked() {
                // TODO: Mostrar menú de componentes
            }
        });
    }
}

impl Default for InspectorPanel {
    fn default() -> Self {
        Self::new()
    }
}

/// Panel de métricas de rendimiento
pub struct MetricsPanel {
    pub show_graphs: bool,
}

impl MetricsPanel {
    pub fn new() -> Self {
        MetricsPanel { show_graphs: false }
    }
    
    pub fn draw(&mut self, ui: &mut egui::Ui, renderer: &dyn crate::render::RenderBackend) {
        let metrics = renderer.get_metrics();
        
        // Header con modo
        ui.horizontal(|ui| {
            match renderer.get_backend_type() {
                crate::render::BackendType::Wgpu => {
                    ui.colored_label(
                        egui::Color32::from_rgb(255, 200, 0),
                        "⚡ MODO BESTIA"
                    );
                }
                crate::render::BackendType::Raylib => {
                    ui.colored_label(
                        egui::Color32::from_rgb(100, 200, 255),
                        "📱 MODO DINÁMICO"
                    );
                }
            }
        });
        
        ui.separator();
        
        // Métricas principales
        ui.horizontal(|ui| {
            ui.label("💾 RAM:");
            ui.label(format!("{:.0} MB", metrics.ram_mb));
        });
        
        ui.horizontal(|ui| {
            ui.label("⚙️ CPU:");
            ui.label(format!("{:.1}%", metrics.cpu_percent));
        });
        
        ui.horizontal(|ui| {
            ui.label("🎮 GPU:");
            ui.label(format!("{:.1}%", metrics.gpu_percent));
        });
        
        ui.horizontal(|ui| {
            ui.label("🎯 FPS:");
            let fps_color = if metrics.fps >= 58 {
                egui::Color32::GREEN
            } else if metrics.fps >= 30 {
                egui::Color32::YELLOW
            } else {
                egui::Color32::RED
            };
            ui.colored_label(fps_color, format!("{}", metrics.fps));
            ui.label(format!("({:.1} ms)", metrics.frame_time_ms));
        });
        
        ui.horizontal(|ui| {
            ui.label("📊 Draw Calls:");
            ui.label(format!("{}", metrics.draw_calls));
            
            ui.label("  🔺 Polígonos:");
            ui.label(format!("{:.1}k", metrics.polygons as f32 / 1000.0));
        });
    }
}

impl Default for MetricsPanel {
    fn default() -> Self {
        Self::new()
    }
}
