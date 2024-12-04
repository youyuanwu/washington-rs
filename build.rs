#[cfg(target_os = "windows")]
fn main() {
    println!("cargo:rerun-if-changed=.windows/winmd/Microsoft.States.winmd");
    println!("cargo:rerun-if-changed=build.rs");

    windows_bindgen::bindgen([
        "--in",
        ".windows/winmd/Microsoft.States.winmd",
        "--out",
        "src/bindings.rs",
        "--filter",
        "Microsoft.States",
    ]);
}

#[cfg(target_os = "linux")]
fn main() {}
