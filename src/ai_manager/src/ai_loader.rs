use std::{fs, sync::Arc, path::{Path, PathBuf}, process::{exit, Command}, ffi::OsStr};
use board::board::Board;
use libloading::{Library, Symbol};

/// Loads an AI function from a compiled shared library.
///
/// # Arguments
/// - `ai_file` - Path to the compiled AI library file.
///
/// # Returns
/// - `Some((Arc<Library>, Box<dyn Fn(&Board, char) -> Option<((usize, usize), (usize, usize))>>, String))`
///   if the AI function is successfully loaded.
/// - `None` if the library could not be loaded.
pub fn load_ai_function(ai_file: PathBuf) -> Option<(Arc<Library>, Box<dyn Fn(&Board, char) -> Option<((usize, usize), (usize, usize))> + Send + Sync>, String)> {

    println!("🤖 Chargement de l'IA depuis {:?}", ai_file);

    let ai_name = ai_file.file_stem().and_then(OsStr::to_str).unwrap_or("unknown_ai").to_string();

    // Load the shared library dynamically
    let lib = Arc::new(unsafe { Library::new(&ai_file).ok()? });

    // SAFELY load the function pointer before returning the closure
    let func: Symbol<unsafe fn(&Board, char) -> Option<((usize, usize), (usize, usize))>> =
        unsafe { lib.get(b"get_ai_move").ok()? };

    // Store a raw function pointer to avoid borrowing issues
    let func_ptr = *func;

    // Create a closure that calls the function
    let ai_function = unsafe{move |board: &Board, player: char| func_ptr(board, player)};

    println!("✅ IA chargée depuis {:?}", ai_file);
    
    Some((lib, Box::new(ai_function), ai_name))
}

/// Loads all AI functions from the `src/ai_folder` directory.
/// Looks for the dynamics library ie: .so (linux), .dylib (MacOS), .dll (windows) files.
///
/// # Returns
/// - `Some(Vec<Option<(Arc<Library>, Box<dyn Fn(&Board, char) -> Option<((usize, usize), (usize, usize))>>, String)>>)`
///   containing all loaded AI functions.
/// - `None` if no AI functions are found or loading fails.
pub fn load_ai_functions() -> Option<Vec<Option<(Arc<Library>, Box<dyn Fn(&Board, char) -> Option<((usize, usize), (usize, usize))> + Send + Sync>, String)>>> {
    let ai_folder = "src/ai_folder/";
    let mut ai_functions_data = Vec::new();

    if let Ok(entries) = fs::read_dir(ai_folder) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let target_path = path.join("target/release");

                if let Ok(files) = fs::read_dir(&target_path) {
                    for file in files.flatten() {
                        let file_path = file.path();
                        if file_path.is_file() {
                            let ext = file_path.extension().and_then(|e| e.to_str());

                            #[cfg(target_os = "windows")]
                            let is_dynamic_lib = ext == Some("dll");

                            #[cfg(target_os = "linux")]
                            let is_dynamic_lib = ext == Some("so");

                            #[cfg(target_os = "macos")]
                            let is_dynamic_lib = ext == Some("dylib");

                            if is_dynamic_lib {
                                let ai_data = load_ai_function(file_path);
                                if ai_data.is_none() {
                                    eprintln!("❌ Erreur Lecture Data IA");
                                    exit(1);
                                }
                                ai_functions_data.push(ai_data);
                            }
                        }
                    }
                }
            }
        }
    }

    // Returning libraries and ai_functions as separate vectors
    Some(ai_functions_data)
}


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