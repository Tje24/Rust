// build.rs - Script de build para configuración específica

fn main() {
    // Configuración específica para Android
    #[cfg(target_os = "android")]
    {
        println!("cargo:rustc-link-lib=android");
        println!("cargo:rustc-link-lib=log");
        println!("cargo:rustc-link-lib=EGL");
        println!("cargo:rustc-link-lib=GLESv3");
    }

    // Recopilar shaders para incluir en el binario
    println!("cargo:rerun-if-changed=shaders/");
    println!("cargo:rerun-if-changed=assets/");
}
