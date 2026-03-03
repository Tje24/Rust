// src/core/editor_app.rs - Aplicación principal del editor

use super::{EditorMode, ViewportMode, RenderMode, SceneManager};
use crate::render::RenderBackend;
use crate::ui::EditorUI;

/// Aplicación principal del editor
pub struct EditorApp {
    /// Estado actual del editor
    pub mode: EditorMode,
    
    /// Modo del viewport
    pub viewport_mode: ViewportMode,
    
    /// Modo de renderizado
    pub render_mode: RenderMode,
    
    /// Gestor de escenas
    pub scene_manager: SceneManager,
    
    /// UI del editor
    pub ui: EditorUI,
    
    /// Backend de renderizado (inicializado después)
    pub renderer: Option<Box<dyn RenderBackend>>,
    
    /// Ruta del proyecto actual
    pub project_path: Option<std::path::PathBuf>,
    
    /// ¿Hay cambios sin guardar?
    pub modified: bool,
}

impl EditorApp {
    /// Crear una nueva instancia del editor
    pub fn new() -> Self {
        log::info!("Creando EditorApp...");
        
        EditorApp {
            mode: EditorMode::Edit,
            viewport_mode: ViewportMode::Mode3D,
            render_mode: RenderMode::Dinamico,
            scene_manager: SceneManager::new(),
            ui: EditorUI::new(),
            renderer: None,
            project_path: None,
            modified: false,
        }
    }
    
    /// Inicializar el renderer después de crear la instancia
    pub fn init_renderer(&mut self) {
        log::info!("Inicializando renderer...");
        
        // Por ahora, usar raylib como fallback
        // En el futuro, detectar capacidades del hardware
        self.renderer = Some(Box::new(
            crate::render::RaylibBackend::new()
        ));
        
        log::info!("Renderer inicializado: {:?}", self.render_mode);
    }
    
    /// Actualizar lógica del editor
    pub fn update(&mut self, delta: f32) {
        // Actualizar escena si está cargada
        if let Some(_scene) = &self.scene_manager.current_scene {
            // Aquí irá la lógica de actualización
        }
        
        // Actualizar UI
        self.ui.update(delta);
    }
    
    /// Renderizar el frame
    pub fn render(&mut self) {
        if let Some(renderer) = &mut self.renderer {
            renderer.begin_frame();
            
            if let Some(scene) = &self.scene_manager.current_scene {
                renderer.render_scene(scene);
            }
            
            renderer.end_frame();
        }
    }
    
    /// Cambiar entre modo edición y modo juego
    pub fn toggle_play_mode(&mut self) {
        self.mode = match self.mode {
            EditorMode::Edit => {
                log::info!("▶ Entrando en modo juego");
                EditorMode::Play
            }
            EditorMode::Play => {
                log::info!("⏹ Volviendo a modo edición");
                EditorMode::Edit
            }
        };
    }
    
    /// Cambiar modo del viewport
    pub fn set_viewport_mode(&mut self, mode: ViewportMode) {
        log::info!("Cambiando viewport a: {:?}", mode);
        self.viewport_mode = mode;
    }
    
    /// Cambiar modo de renderizado
    pub fn set_render_mode(&mut self, mode: RenderMode) {
        if self.render_mode != mode {
            log::info!("Cambiando render mode de {:?} a {:?}", self.render_mode, mode);
            self.render_mode = mode;
            // Reinicializar renderer con el nuevo backend
            self.init_renderer();
        }
    }
    
    /// Cargar una escena desde archivo
    pub fn load_scene(&mut self, path: &str) -> Result<(), anyhow::Error> {
        log::info!("Cargando escena: {}", path);
        
        self.scene_manager.open_scene(std::path::PathBuf::from(path))?;
        self.project_path = Some(std::path::PathBuf::from(path));
        self.modified = false;
        
        Ok(())
    }
    
    /// Guardar escena actual
    pub fn save_scene(&mut self) -> Result<(), anyhow::Error> {
        if let Some(path) = &self.project_path {
            log::info!("Guardando escena: {:?}", path);
            self.scene_manager.save_scene(path)?;
            self.modified = false;
            Ok(())
        } else {
            Err(anyhow::anyhow!("No hay escena cargada"))
        }
    }
    
    /// Crear nueva escena
    pub fn new_scene(&mut self, name: &str) {
        log::info!("Creando nueva escena: {}", name);
        self.scene_manager.create_new_scene(name);
        self.modified = true;
    }
}

impl Default for EditorApp {
    fn default() -> Self {
        Self::new()
    }
}

impl eframe::App for EditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Inicializar renderer si no está listo
        if self.renderer.is_none() {
            self.init_renderer();
        }
        
        // Panel central
        egui::CentralPanel::default().show(ctx, |ui| {
            self.ui.draw_viewport(ui, self);
        });
        
        // Panel izquierdo (archivos)
        egui::SidePanel::left("files_panel")
            .width_range(150.0..=300.0)
            .show(ctx, |ui| {
                self.ui.draw_files_panel(ui, self);
            });
        
        // Panel derecho (inspector)
        egui::SidePanel::right("inspector_panel")
            .width_range(200.0..=350.0)
            .show(ctx, |ui| {
                self.ui.draw_inspector(ui, self);
            });
        
        // Panel inferior (animación, tilemap, etc)
        egui::TopBottomPanel::bottom("bottom_panel")
            .height_range(100.0..=200.0)
            .show(ctx, |ui| {
                self.ui.draw_bottom_panel(ui, self);
            });
        
        // Solicitar repaint continuo (para preview en tiempo real)
        if self.mode == EditorMode::Play {
            ctx.request_repaint();
        }
    }
}
