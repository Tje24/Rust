// src/lib.rs - Punto de entrada principal para Android

pub mod core;
pub mod render;
pub mod ui;
pub mod scripting;
pub mod assets;
pub mod ecs;
pub mod animation;
pub mod tilemap;
pub mod physics;
pub mod export;

#[cfg(target_os = "android")]
use android_logger::Config;
#[cfg(target_os = "android")]
use log::Level;

/// Inicialización específica para Android
#[cfg(target_os = "android")]
fn init_android_logging() {
    android_logger::init_once(
        Config::default()
            .with_min_level(Level::Debug)
            .with_tag("rust_editor")
    );
    log::info!("🦀 Rust Editor iniciado en Android");
}

/// Inicialización para desktop
#[cfg(not(target_os = "android"))]
fn init_desktop_logging() {
    env_logger::init();
    log::info!("🦀 Rust Editor iniciado en Desktop");
}

/// Punto de entrada principal para Android
#[cfg(target_os = "android")]
#[no_mangle]
pub extern "C" fn android_main(app: android_activity::AndroidApp) {
    init_android_logging();

    log::info!("Iniciando editor...");

    // Inicializar el core del editor
    let mut editor = core::EditorApp::new();
    editor.init_renderer();

    // Bucle principal (simplificado por ahora)
    // En implementación real, esto integrará con egui + render backend
    loop {
        // Verificar si la actividad debe cerrarse
        if app.poll_events().is_some() {
            break;
        }

        // Actualizar lógica
        editor.update(0.016);

        // Renderizar
        editor.render();
    }
}

/// Función de creación del editor (compartida)
pub fn create_editor() -> core::EditorApp {
    #[cfg(not(target_os = "android"))]
    init_desktop_logging();

    core::EditorApp::new()
}
