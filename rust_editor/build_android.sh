#!/bin/bash
# build_android.sh - Script para compilar Rust Editor para Android

set -e

# Configuración
VERSION="0.1.0"
BUILD_DIR="target/android-build"
APK_DIR="$BUILD_DIR/apk"

# Colores
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}  Rust Editor - Build Android v${VERSION}  ${NC}"
echo -e "${GREEN}========================================${NC}"

# Verificar variables de entorno
if [ -z "$ANDROID_HOME" ]; then
    echo -e "${RED}ERROR: ANDROID_HOME no está definido${NC}"
    echo "Exporta: export ANDROID_HOME=\$HOME/Android/Sdk"
    exit 1
fi

# Verificar NDK
if [ -z "$ANDROID_NDK_HOME" ]; then
    echo -e "${YELLOW}WARNING: ANDROID_NDK_HOME no está definido${NC}"
    echo "Usando NDK desde ANDROID_HOME..."
    export ANDROID_NDK_HOME="$ANDROID_HOME/ndk"
fi

# Crear directorio de build
mkdir -p $BUILD_DIR

# Añadir targets si no existen
echo -e "${YELLOW}[1/6] Configurando targets...${NC}"
rustup target add aarch64-linux-android 2>/dev/null || true
rustup target add armv7-linux-androideabi 2>/dev/null || true

# Compilar para ARM64 (dispositivos modernos)
echo -e "${YELLOW}[2/6] Compilando para ARM64 (aarch64-linux-android)...${NC}"
cargo build --release --target aarch64-linux-android

# Compilar para ARMv7 (dispositivos 2GB RAM)
echo -e "${YELLOW}[3/6] Compilando para ARMv7 (armv7-linux-androideabi)...${NC}"
cargo build --release --target armv7-linux-androideabi

# Crear estructura de APK
echo -e "${YELLOW}[4/6] Creando estructura de APK...${NC}"
mkdir -p $APK_DIR/lib/arm64-v8a
mkdir -p $APK_DIR/lib/armeabi-v7a
mkdir -p $APK_DIR/assets

# Copiar librerías nativas
cp target/aarch64-linux-android/release/librust_editor.so $APK_DIR/lib/arm64-v8a/
cp target/armv7-linux-androideabi/release/librust_editor.so $APK_DIR/lib/armeabi-v7a/

# Copiar AndroidManifest.xml
cp android/AndroidManifest.xml $APK_DIR/

# Copiar recursos
cp -r android/res $APK_DIR/

# Copiar assets de ejemplo
cp -r examples/* $APK_DIR/assets/ 2>/dev/null || true

# Crear APK sin firmar
echo -e "${YELLOW}[5/6] Empaquetando APK...${NC}"
cd $APK_DIR

# Usar aapt2 si está disponible, sino zip
if command -v aapt2 &> /dev/null; then
    aapt2 package -o ../editor-unsigned.apk \
        --manifest AndroidManifest.xml \
        -R res/ \
        assets/ \
        lib/
else
    zip -r ../editor-unsigned.apk *
fi

cd ../..

# Zipalign (optimizar APK)
echo -e "${YELLOW}[6/6] Optimizando con zipalign...${NC}"
if command -v zipalign &> /dev/null; then
    zipalign -f -p 4 $BUILD_DIR/editor-unsigned.apk $BUILD_DIR/editor-aligned.apk
else
    echo -e "${YELLOW}WARNING: zipalign no encontrado, saltando optimización${NC}"
    cp $BUILD_DIR/editor-unsigned.apk $BUILD_DIR/editor-aligned.apk
fi

# Firmar con debug key (solo para desarrollo)
echo -e "${GREEN}Generando APK de debug...${NC}"
if command -v apksigner &> /dev/null; then
    apksigner sign \
        --ks ~/.android/debug.keystore \
        --ks-key-alias androiddebugkey \
        --ks-pass pass:android \
        --key-pass pass:android \
        --out $BUILD_DIR/rust-editor-debug.apk \
        $BUILD_DIR/editor-aligned.apk
    
    # Verificar firma
    apksigner verify $BUILD_DIR/rust-editor-debug.apk
else
    echo -e "${YELLOW}WARNING: apksigner no encontrado${NC}"
    cp $BUILD_DIR/editor-aligned.apk $BUILD_DIR/rust-editor-debug.apk
fi

# Limpiar archivos temporales
rm -f $BUILD_DIR/editor-unsigned.apk $BUILD_DIR/editor-aligned.apk

# Mostrar resultado
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}✅ ¡BUILD COMPLETADO!${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""
echo -e "APK generado: ${YELLOW}$BUILD_DIR/rust-editor-debug.apk${NC}"
echo ""
ls -lh $BUILD_DIR/rust-editor-debug.apk
echo ""
echo -e "Para instalar en dispositivo:"
echo -e "  ${YELLOW}adb install $BUILD_DIR/rust-editor-debug.apk${NC}"
echo ""
