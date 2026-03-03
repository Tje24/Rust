# Documentación de Rust Editor

## Inicio Rápido

### 1. Prerequisites

- Rust 1.75+
- Android SDK y NDK
- Herramientas de build de Android (build-tools, platform-tools)

### 2. Configurar variables de entorno

```bash
export ANDROID_HOME=$HOME/Android/Sdk
export ANDROID_NDK_HOME=$ANDROID_HOME/ndk
export PATH=$PATH:$ANDROID_HOME/platform-tools
export PATH=$PATH:$ANDROID_HOME/build-tools/34.0.0
```

### 3. Build para Android

```bash
cd rust_editor
chmod +x build_android.sh
./build_android.sh
```

### 4. Instalar en dispositivo

```bash
adb install target/android-build/rust-editor-debug.apk
```

## Estructura del Proyecto

```
rust_editor/
├── src/
│   ├── lib.rs                 # Punto de entrada principal
│   ├── bin/
│   │   └── editor_desktop.rs  # Versión desktop para desarrollo
│   ├── core/                  # Lógica principal del editor
│   │   ├── mod.rs
│   │   ├── editor_app.rs      # Aplicación principal
│   │   ├── scene_manager.rs   # Gestión de escenas
│   │   └── editor_state.rs    # Estado del editor
│   ├── render/                # Backends de renderizado
│   │   ├── mod.rs
│   │   ├── render_backend.rs  # Trait común
│   │   └── raylib_backend.rs  # Implementación raylib
│   ├── ui/                    # Interfaz con egui
│   │   ├── mod.rs
│   │   ├── editor_ui.rs       # UI principal
│   │   └── panels.rs          # Paneles individuales
│   ├── scripting/             # Sistema de scripting TOML
│   │   ├── mod.rs
│   │   ├── scene.rs           # Estructura de escena
│   │   ├── node.rs            # Nodos de escena
│   │   ├── components.rs      # Componentes ECS
│   │   └── behavior.rs        # Comportamientos
│   ├── animation/             # Sistema de animación
│   ├── tilemap/               # Editor de tilemaps
│   ├── physics/               # Integración de físicas
│   ├── assets/                # Gestión de assets
│   ├── ecs/                   # Entity-Component-System
│   └── export/                # Exportación a APK
├── android/                   # Configuración Android
│   ├── AndroidManifest.xml
│   └── res/
├── assets/                    # Recursos del editor
├── examples/                  # Ejemplos de escenas
├── Cargo.toml                 # Dependencias
└── build_android.sh           # Script de build
```

## Formato de Escena (TOML)

Ver `examples/scene_example.toml` para un ejemplo completo.

## Comportamientos Disponibles

### Oscillate
Hace que un objeto oscile en un eje.

```toml
[behavior]
type = "oscillate"
axis = "y"
speed = 2.0
amplitude = 0.5
```

### Patrol
Patrulla entre waypoints.

```toml
[behavior]
type = "patrol"
speed = 2.0
wait_time = 1.0
waypoints = [[x, y, z], ...]
```

## Arquitectura

### Doble Backend

- **Modo Dinámico (raylib)**: OpenGL ES 2.0+, compatible con dispositivos desde 2GB RAM
- **Modo Bestia (wgpu)**: Vulkan/Metal, para dispositivos gama alta

### Una Escena Activa

El editor mantiene solo una escena cargada en memoria para optimizar RAM.

### Scripting Basado en Datos

Todos los comportamientos y configuraciones se definen en TOML/JSON, no en código.

## Licencia

MIT
