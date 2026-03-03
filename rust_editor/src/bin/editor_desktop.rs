// src/bin/editor_desktop.rs - Punto de entrada para desktop (desarrollo)

use rust_editor::core;
use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    // Inicializar logging
    env_logger::init();
    log::info!("🚀 Iniciando Rust Editor en Desktop");

    // Configuración de la ventana
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 800.0])
            .with_min_inner_size([800.0, 600.0])
            .with_title("Rust Editor v0.1.0")
            .with_icon(load_icon()),
        ..Default::default()
    };

    // Crear y ejecutar la aplicación
    eframe::run_native(
        "Rust Editor",
        options,
        Box::new(|cc| {
            // Configurar fuentes y estilos
            setup_egui(&cc.egui_ctx);
            
            // Crear el editor
            Ok(Box::new(core::EditorApp::new()))
        }),
    )
}

/// Cargar ícono de la aplicación
fn load_icon() -> egui::IconData {
    // Ícono por defecto (32x32 rgba)
    let rgba = vec![255; 32 * 32 * 4];
    egui::IconData {
        rgba,
        width: 32,
        height: 32,
    }
}

/// Configurar egui (fuentes, estilos, temas)
fn setup_egui(ctx: &egui::Context) {
    // Tema oscuro por defecto
    ctx.set_visuals(egui::Visuals::dark());

    // Configurar fuentes
    let mut fonts = egui::FontDefinitions::default();
    
    // Insertar fuente personalizada si existe
    // fonts.font_data.insert(...)
    
    ctx.set_fonts(fonts);

    // Configurar estilo
    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = egui::vec2(8.0, 8.0);
    style.spacing.button_padding = egui::vec2(8.0, 4.0);
    ctx.set_style(style);
}
