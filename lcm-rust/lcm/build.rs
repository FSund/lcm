fn main() {
    // Use pkg-config to find LCM library
    if let Ok(library) = pkg_config::Config::new().probe("lcm") {
        // pkg-config automatically adds the necessary link flags
        for link_path in &library.link_paths {
            println!("cargo:rustc-link-search=native={}", link_path.display());
        }
        for lib in &library.libs {
            println!("cargo:rustc-link-lib={}", lib);
        }
    } else {
        // Fallback: try to link manually
        println!("cargo:rustc-link-lib=lcm");
        println!("cargo:rustc-link-lib=glib-2.0");
    }
}
