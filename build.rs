use std::path::PathBuf;

/// Returns the root path of the wasm workspace.
fn get_wasm_workspace_root() -> PathBuf {
    let mut out_dir = build_helper::out_dir();

    loop {
        match out_dir.parent() {
            Some(parent) if out_dir.ends_with("build") => return parent.to_path_buf(),
            _ => {
                if !out_dir.pop() {
                    break;
                }
            }
        }
    }

    panic!(
        "Could not find target dir in: {}",
        build_helper::out_dir().display()
    )
}

fn main() {
    println!("cargo:rerun-if-env-changed=SKIP_UI_BUILD");

    let wasm_workspace_root = get_wasm_workspace_root();
    let wasm_workspace = wasm_workspace_root.join("wbuild");
    let dist_dir = build_helper::out_dir().join("dist");

    let _ = std::fs::remove_dir_all(dist_dir.clone());
    std::fs::create_dir_all(dist_dir.clone()).unwrap();

    if std::env::var_os("SKIP_UI_BUILD").is_some() {
        std::fs::write(
            dist_dir.clone().join("index.html"),
            "<p>UI build was skipped</p>",
        )
        .unwrap();
    } else {
        println!("cargo:rerun-if-changed=ui");

        let mut command = std::process::Command::new("trunk");
        if build_helper::debug() {
            command.args(["build", "--dist", &dist_dir.to_string_lossy()]);
        } else {
            command.args(["build", "--release", "--dist", &dist_dir.to_string_lossy()]);
        }
        command.env("CARGO_TARGET_DIR", wasm_workspace);
        command.current_dir(build_helper::cargo::manifest::dir().join("ui"));

        let trunk_status = command.spawn().unwrap().wait().unwrap();
        assert!(trunk_status.success());
    }
}
