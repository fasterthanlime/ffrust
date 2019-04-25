use std::path::PathBuf;

fn main() {
    let lib_path = std::env::var("FFMPEG_DIR").expect("FFMPEG_DIR must be set");
    let lib_path: PathBuf = lib_path.into();
    let lib_path = lib_path.join("lib");

    println!(
        "cargo:rustc-link-search=native={}",
        lib_path.to_str().unwrap()
    );

    let target = std::env::var("TARGET").unwrap();
    let x264_lib = if target.contains("windows-msvc") {
        // x264's build scripts produce "libx264.lib" for
        // some reason. oh well.
        "libx264"
    } else {
        "x264"
    };

    let static_libs = vec!["avutil", "avformat", "avcodec", "swscale", "swresample", x264_lib];
    for lib in static_libs {
        println!("cargo:rustc-link-lib=static={}", lib);
    }

    let dynamic_libs = vec!["bcrypt", "user32"];
    for lib in dynamic_libs {
        println!("cargo:rustc-link-lib=dylib={}", lib);
    }
}
