// src/export/exporter.rs - Exportador de proyectos

use super::{ExportConfig, Architecture, ExportError};
use crate::core::EditorApp;

/// Exportador de proyectos a APK
pub struct ProjectExporter {
    pub config: ExportConfig,
}

impl ProjectExporter {
    pub fn new(config: ExportConfig) -> Self {
        ProjectExporter { config }
    }
    
    pub fn export(&self, _app: &mut EditorApp) -> Result<String, ExportError> {
        log::info!("Exportando proyecto...");
        
        // En implementación real:
        // 1. Recopilar assets
        // 2. Compilar para cada arquitectura
        // 3. Empaquetar APK
        // 4. Firmar APK
        
        for arch in &self.config.architectures {
            log::info!("  Compilando para {:?}", arch);
        }
        
        Ok(format!("{}/{}.apk", self.config.output_path, self.config.package_name))
    }
    
    fn compile_for_architecture(&self, arch: &Architecture) -> Result<(), ExportError> {
        let target = match arch {
            Architecture::Arm64V8a => "aarch64-linux-android",
            Architecture::ArmeabiV7a => "armv7-linux-androideabi",
            Architecture::X86_64 => "x86_64-linux-android",
        };
        
        log::debug!("Target: {}", target);
        
        // TODO: Ejecutar cargo build --release --target <target>
        
        Ok(())
    }
}

impl Default for ProjectExporter {
    fn default() -> Self {
        Self::new(ExportConfig {
            output_path: "./build".to_string(),
            package_name: "com.tueditor.game".to_string(),
            version_name: "1.0.0".to_string(),
            version_code: 1,
            architectures: vec![
                Architecture::Arm64V8a,
                Architecture::ArmeabiV7a,
            ],
        })
    }
}
