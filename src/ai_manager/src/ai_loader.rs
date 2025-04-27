use std::{fs, path::{Path}, process::{Command}};

/// Scan `src/ai_folder` for sub-crates, locate their compiled dynamic libraries,
/// and return a vector of (AI name, dynamic library path) tuples.
pub fn find_all_ai_plugin_paths() -> Vec<(String, String)> {
    let mut results = Vec::new();
    let base = Path::new("src/ai_folder");

    if let Ok(entries) = fs::read_dir(base) {
        for entry in entries.flatten() {
            let dir = entry.path();
            if dir.is_dir() {
                if let Some(ai_name) = dir.file_name().and_then(|s| s.to_str()) {
                    let release_dir = dir.join("target").join("release");
                    if let Ok(files) = fs::read_dir(&release_dir) {
                        for file in files.flatten() {
                            let path = file.path();
                            if path.is_file() {
                                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                                    let is_dynlib = cfg!(target_os = "windows") && ext.eq_ignore_ascii_case("dll")
                                        || cfg!(target_os = "linux") && ext == "so"
                                        || cfg!(target_os = "macos") && ext == "dylib";
                                    if is_dynlib {
                                        results.push((
                                            ai_name.to_string(),
                                            path.to_string_lossy().into_owned(),
                                        ));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    results
}

/// Compiles all AI packages found in the `src/ai_folder` directory.
///
/// # Returns
/// - `Ok(())` if all AI packages compile successfully.
/// - `Err(String)` if an error occurs during compilation.
pub fn compile_all_ai() -> Result<(), String> {
    let ai_folder = "src/ai_folder";

    // Read AI package directories
    let entries = fs::read_dir(ai_folder).map_err(|e| format!("❌ Failed to read AI folder: {}", e))?;

    for entry in entries.flatten() {
        let path = entry.path();

        if path.is_dir() {
            let cargo_toml = path.join("Cargo.toml");

            if cargo_toml.exists() {
                println!("🤖 Compiling AI: {:?}", path);

                // Run cargo build --release inside the AI package
                let output = Command::new("cargo")
                    .arg("build")
                    .arg("--release")
                    .current_dir(&path)
                    .output()
                    .map_err(|e| format!("❌ Failed to execute cargo build: {}", e))?;

                if !output.status.success() {
                    return Err(format!(
                        "❌ Failed to compile {:?}: {}",
                        path,
                        String::from_utf8_lossy(&output.stderr)
                    ));
                }
            }
        }
    }

    Ok(())
}