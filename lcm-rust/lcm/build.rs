fn main() {
    // Use pkg-config to find LCM library
    let mut config = pkg_config::Config::new();
    config.atleast_version("1.5.0");

    // Only use static linking on Windows
    #[cfg(target_os = "windows")]
    config.statik(true);

    if let Ok(library) = config.probe("lcm") {
        // pkg-config automatically adds the necessary link flags
        for link_path in &library.link_paths {
            println!("cargo:rustc-link-search=native={}", link_path.display());
        }
        for lib in &library.libs {
            println!("cargo:rustc-link-lib={}", lib);
        }
    } else {
        // Fallback: try to link manually
        println!(
            "cargo:warning=Falling back to manual linking. LCM library not found via pkg-config."
        );
        println!("cargo:rustc-link-lib=lcm");
        println!("cargo:rustc-link-lib=glib-2.0");
    }
}
