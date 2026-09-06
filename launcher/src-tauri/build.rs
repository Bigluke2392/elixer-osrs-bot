fn main() {
    // Default build embeds the app manifest (incl. Common Controls v6, which
    // provides TaskDialogIndirect). On the GNU toolchain some mingw-w64
    // distributions auto-link their own default-manifest.o (guarded by an
    // `if-exists` gcc spec), which collides with Tauri's embedded manifest
    // and fails the link with "duplicate resource: type MANIFEST". MSVC has
    // no such object, so only strip the manifest for GNU.
    let attributes = if cfg!(target_env = "gnu") {
        tauri_build::Attributes::new()
            .windows_attributes(tauri_build::WindowsAttributes::new_without_app_manifest())
    } else {
        tauri_build::Attributes::new()
    };

    if let Err(error) = tauri_build::try_build(attributes) {
        panic!("error found during tauri-build: {error:#}");
    }
}
