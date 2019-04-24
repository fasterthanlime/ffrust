use std::path::PathBuf;

fn main() {
    let lib_path = std::env::var("FFMPEG_DIR").expect("FFMPEG_DIR must be set");
    let lib_path: PathBuf = lib_path.into();
    let lib_path = lib_path.join("lib");

    println!("cargo:rustc-link-search=native={}", lib_path.to_str().unwrap());

    let static_libs = vec!["libavutil", "libavformat", "libavcodec", "libswscale", "libx264"];
    for lib in static_libs {
        println!("cargo:rustc-link-lib=static={}", lib);
    }

    let dynamic_libs = vec!["bcrypt", "user32"];
    for lib in dynamic_libs {
        println!("cargo:rustc-link-lib=dylib={}", lib);
    }
}