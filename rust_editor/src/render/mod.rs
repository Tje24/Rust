// src/render/mod.rs - Módulo de renderizado

mod render_backend;
mod wgpu_backend;

pub use render_backend::*;
pub use wgpu_backend::WgpuBackend;

/// Métricas de rendimiento
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    /// Uso de RAM en MB
    pub ram_mb: f32,
    /// Uso de CPU en %
    pub cpu_percent: f32,
    /// Uso de GPU en %
    pub gpu_percent: f32,
    /// Frames por segundo
    pub fps: u32,
    /// Tiempo por frame en ms
    pub frame_time_ms: f32,
    /// Número de draw calls
    pub draw_calls: u32,
    /// Número de polígonos
    pub polygons: u32,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        PerformanceMetrics {
            ram_mb: 0.0,
            cpu_percent: 0.0,
            gpu_percent: 0.0,
            fps: 60,
            frame_time_ms: 16.67,
            draw_calls: 0,
            polygons: 0,
        }
    }
}
