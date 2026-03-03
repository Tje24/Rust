# Rust Editor 🦀

Editor de juegos visual para Android construido con Rust + wgpu + raylib + egui.

## 🎯 Objetivos

- **Una escena activa**: Máxima eficiencia en móvil (2GB RAM+)
- **Doble backend**: wgpu (rendimiento) + raylib (compatibilidad)
- **Scripting TOML/JSON**: Datos portables, seguros, IA-amigables
- **Exportación directa a APK**: Sin pasos intermedios

## 🚀 Quick Start

### Desktop (desarrollo)

```bash
cargo run --bin editor_desktop
```

### Android

```bash
# Añadir targets
rustup target add aarch64-linux-android armv7-linux-androideabi

# Compilar
./build_android.sh

# Instalar
adb install target/android-build/rust_editor-debug.apk
```

## 📁 Estructura

```
rust_editor/
├── src/
│   ├── core/          # EditorApp, SceneManager
│   ├── render/        # WgpuBackend, RaylibBackend
│   ├── ui/            # Paneles egui
│   ├── scripting/     # Comportamientos TOML
│   └── ...
├── assets/            # Recursos del editor
├── examples/          # Proyectos de ejemplo
└── android/           # Configuración Android
```

## 📋 Roadmap

- [ ] Fase 0: Prototipo (ventana + triángulo)
- [ ] Fase 1: Core (escena TOML + viewport)
- [ ] Fase 2: UI completa
- [ ] Fase 3: Animación
- [ ] Fase 4: Tilemap
- [ ] Fase 5: Físicas
- [ ] Fase 6: Exportación APK

## 📄 Licencia

MIT
