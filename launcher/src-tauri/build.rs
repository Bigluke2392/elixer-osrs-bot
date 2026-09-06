fn main() {
    if std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("gnu") {
        // osrs-bot: some mingw-w64 distributions (e.g. WinLibs) auto-link
        // their own default-manifest.o (via an `if-exists` gcc *endfile*
        // spec) for long-path awareness. It collides with Tauri's embedded
        // manifest (both claim RT_MANIFEST resource ID 1), failing the link
        // with "duplicate resource: type MANIFEST".
        //
        // We can't drop Tauri's manifest instead (as a previous version of
        // this file did) — it's what activates Common Controls v6, which
        // TaskDialogIndirect is statically imported from. Without it, the
        // loader binds that import against the old v5 comctl32.dll, which
        // doesn't export it, and the exe fails to even start with
        // STATUS_ENTRYPOINT_NOT_FOUND.
        //
        // So instead, override gcc's *endfile* spec to drop just the
        // conflicting default-manifest.o fragment (reproduced verbatim from
        // `gcc -dumpspecs` otherwise), letting Tauri's manifest win.
        let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR not set");
        let specs_path = std::path::Path::new(&out_dir).join("no-default-manifest.spec");
        std::fs::write(
            &specs_path,
            "*endfile:\n\
             %{mdaz-ftz:crtfastmath.o%s;Ofast|ffast-math|funsafe-math-optimizations:%{!shared:%{!mno-daz-ftz:crtfastmath.o%s}}} \
             %{fvtable-verify=none:%s;fvtable-verify=preinit:vtv_end.o%s;fvtable-verify=std:vtv_end.o%s} \
             crtend.o%s\n",
        )
        .expect("failed to write linker specs override");
        println!("cargo:rustc-link-arg=-specs={}", specs_path.display());
    }

    if let Err(error) = tauri_build::try_build(tauri_build::Attributes::new()) {
        panic!("error found during tauri-build: {error:#}");
    }
}
