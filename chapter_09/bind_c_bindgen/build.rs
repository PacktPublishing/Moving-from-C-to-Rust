use std::path::PathBuf;

fn main() {
    let dst = cmake::Config::new("SDL2-2.30.9")
        .profile("Release")
        .define("SDL_SHARED", "OFF")
        .define("SDL_STATIC", "ON")
        .build();

    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );

    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=static=SDL2main");
        println!("cargo:rustc-link-lib=static=SDL2-static");
    } else {
        println!("cargo:rustc-link-lib=static=SDL2");
    }

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}
