fn main() {
    println!("cargo:rerun-if-env-changed=MORUM_API_PREFIX");

    if let Some(api_prefix) = std::env::var_os("MORUM_API_PREFIX") {
        std::fs::write(build_helper::out_dir().join("api_prefix.txt"), api_prefix.into_string().unwrap()).unwrap();
    } else {
        std::fs::write(build_helper::out_dir().join("api_prefix.txt"), "").unwrap();
    }
}
