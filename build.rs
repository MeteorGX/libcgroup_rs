
fn main(){
    let mode = std::env::var("CGROUP_LINKAGE")
        .unwrap_or("dylib".to_owned());

    if let Ok(cgroup_link_path) = std::env::var("CGROUP_LIBRARY_PATH") {
        for path in std::env::split_paths(&cgroup_link_path) {
            println!("cargo:rustc-link-search=native={}", &path.to_string_lossy());
        }
    }

    if let Ok(e_libs) = std::env::var("CGROUP_LIBS") {
        // Link against the libraries in CGROUP_LIBS, multiple
        // libraries can specified, separated by semicolon(;)
        for lib in e_libs.split(";") {
            println!("cargo:rustc-link-lib={}={}", mode, lib);
        }
    } else {
        println!("cargo:rustc-link-lib={}=cgroup", mode);
    }
}