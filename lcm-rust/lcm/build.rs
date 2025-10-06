use std::env;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    // Path to the LCM C source directory (relative to the project root)
    let lcm_src_dir = PathBuf::from(&manifest_dir)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("lcm");

    // Generate lcm_export.h header first
    let export_header_path = PathBuf::from(&out_dir).join("lcm_export.h");
    std::fs::write(&export_header_path, generate_export_header()).unwrap();

    // Generate lcm_c_namespace.h from template
    let namespace_header_in = lcm_src_dir.join("lcm_c_namespace.h.in");
    let namespace_header_out = PathBuf::from(&out_dir).join("lcm_c_namespace.h");
    let template =
        std::fs::read_to_string(&namespace_header_in).expect("Failed to read lcm_c_namespace.h.in");
    // Substitute @LCM_C_NAMESPACE@ with default value "lcm"
    let namespace_value = "lcm"; // You may want to make this configurable
    let header = template.replace("@LCM_C_NAMESPACE@", namespace_value);
    std::fs::write(&namespace_header_out, header).expect("Failed to write lcm_c_namespace.h");

    let mut build = cc::Build::new();

    // Add LCM source files
    let lcm_sources = [
        "eventlog.c",
        "lcm.c",
        "lcm_file.c",
        "lcm_memq.c",
        "lcm_mpudpm.c",
        "lcm_tcpq.c",
        "lcm_udpm.c",
        "ringbuffer.c",
        "udpm_util.c",
        "lcmtypes/channel_port_map_update_t.c",
        "lcmtypes/channel_to_port_t.c",
    ];

    for source in &lcm_sources {
        build.file(lcm_src_dir.join(source));
    }

    // Add Windows-specific sources if building for Windows
    if target.contains("windows") {
        build.file(lcm_src_dir.join("windows/WinLCM.cpp"));
        build.file(lcm_src_dir.join("windows/WinPorting.cpp"));
    }

    // Include directories
    build.include(&lcm_src_dir);
    build.include(&out_dir); // Include the generated headers directory
    build.include(&out_dir); // Ensure generated lcm_c_namespace.h is found

    // Compiler definitions
    build.define("_FILE_OFFSET_BITS", "64");
    build.define("_LARGEFILE_SOURCE", None);
    build.define("_REENTRANT", None);
    build.define("LCM_STATIC", None);
    build.define("_DEFAULT_SOURCE", None); // For strdup and other POSIX functions
    build.define("_GNU_SOURCE", None); // Additional GNU extensions

    // Platform-specific configurations
    if target.contains("windows") {
        // Windows-specific settings
        build.define("WIN32", None);
        build.define("_WIN32_WINNT", "0x0601"); // Windows 7+

        let glib = vcpkg::find_package("glib-2.0").unwrap();

        // Add include paths from vcpkg
        build.include(&glib.include_paths);
        for lib in glib.libs {
            println!("cargo:rustc-link-lib={}", lib);
        }

        // Link the libraries found by vcpkg
        for link_path in &glib.link_paths {
            println!("cargo:rustc-link-search=native={}", link_path.display());
        }

        for lib_name in &glib.found_names {
            println!("cargo:rustc-link-lib={}", lib_name);
        }

        // Link against Windows libraries
        println!("cargo:rustc-link-lib=ws2_32");
        println!("cargo:rustc-link-lib=iphlpapi");
    } else {
        // Unix-like platforms

        // Try to find glib-2.0 using pkg-config
        match pkg_config::Config::new()
            .atleast_version("2.0")
            .probe("glib-2.0")
        {
            Ok(glib) => {
                // Add GLib include paths and libraries
                for include_path in glib.include_paths {
                    build.include(include_path);
                }
                for lib_path in glib.link_paths {
                    println!("cargo:rustc-link-search=native={}", lib_path.display());
                }
                for lib in glib.libs {
                    println!("cargo:rustc-link-lib={}", lib);
                }
            }
            Err(_) => {
                // Fallback: try common system paths
                build.include("/usr/include/glib-2.0");
                build.include("/usr/lib/x86_64-linux-gnu/glib-2.0/include");
                build.include("/usr/lib64/glib-2.0/include");
                build.include("/usr/lib/glib-2.0/include");

                // Try pkg-config style paths on different distributions
                build.include("/usr/include/glib-2.0");
                build.include("/usr/lib/glib-2.0/include");
                build.include("/usr/local/include/glib-2.0");
                build.include("/usr/local/lib/glib-2.0/include");
                build.include("/opt/homebrew/include/glib-2.0"); // macOS Homebrew
                build.include("/opt/homebrew/lib/glib-2.0/include");

                println!("cargo:rustc-link-lib=glib-2.0");
                println!("cargo:rustc-link-lib=gobject-2.0");
            }
        }

        // Link pthread on Unix-like systems
        println!("cargo:rustc-link-lib=pthread");
    }

    // Set C standard
    build.std("c99");

    // Enable optimization for release builds
    if env::var("PROFILE").unwrap() == "release" {
        build.opt_level(3);
    }

    // Compile the library
    build.compile("lcm");

    // Tell cargo to re-run this script if any of the C source files change
    println!("cargo:rerun-if-changed={}", lcm_src_dir.display());
    for source in &lcm_sources {
        println!(
            "cargo:rerun-if-changed={}",
            lcm_src_dir.join(source).display()
        );
    }

    // Make the include directory available to the Rust code
    println!("cargo:include={}", lcm_src_dir.display());
    println!("cargo:include={}", out_dir);
}

fn generate_export_header() -> &'static str {
    r#"#ifndef LCM_EXPORT_H
#define LCM_EXPORT_H

#ifdef LCM_STATIC
#  define LCM_EXPORT
#  define LCM_NO_EXPORT
#else
#  ifndef LCM_EXPORT
#    ifdef lcm_EXPORTS
        /* We are building this library */
#      define LCM_EXPORT __attribute__((visibility("default")))
#    else
        /* We are using this library */
#      define LCM_EXPORT __attribute__((visibility("default")))
#    endif
#  endif

#  ifndef LCM_NO_EXPORT
#    define LCM_NO_EXPORT __attribute__((visibility("hidden")))
#  endif
#endif

#ifndef LCM_DEPRECATED
#  define LCM_DEPRECATED __attribute__ ((__deprecated__))
#endif

#ifndef LCM_DEPRECATED_EXPORT
#  define LCM_DEPRECATED_EXPORT LCM_EXPORT LCM_DEPRECATED
#endif

#ifndef LCM_DEPRECATED_NO_EXPORT
#  define LCM_DEPRECATED_NO_EXPORT LCM_NO_EXPORT LCM_DEPRECATED
#endif

#if 0 /* DEFINE_NO_DEPRECATED */
#  ifndef LCM_NO_DEPRECATED
#    define LCM_NO_DEPRECATED
#  endif
#endif

#endif /* LCM_EXPORT_H */
"#
}
