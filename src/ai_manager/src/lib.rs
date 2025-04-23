use std::{fs,fs::File, sync::Arc, path::{PathBuf}, process::{exit, Command}, io::{self, Write}, ffi::OsStr};
use libloading::{Library, Symbol};
use board::Board;

/// Creates a new AI by generating its package and source files.
///
/// This function prompts the user for an AI name, sets up the necessary directory structure,
/// and writes an initial `lib.rs` file for the AI.
///
/// # Returns
/// - `Ok(())` if the AI package was created successfully.
/// - `Err(io::Error)` if any file operation fails.
pub fn create_new_ai() -> std::io::Result<()> {
    // Demander le nom de l'IA
    println!("🤖 Entrez le nom de votre IA :");
    let mut ai_name = String::new();
    io::stdin().read_line(&mut ai_name).expect("Échec de la lecture de l'entrée");
    let ai_name = ai_name.trim();

    // Vérification du nom
    if ai_name.is_empty() {
        eprintln!("❌ Le nom de l'IA ne peut pas être vide.");
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Nom d'IA vide"));
    }

    let ai_path = format!("src/ai_folder/{}", ai_name);
    let src_path = format!("{}/src", ai_path);

    create_ai_package(&ai_path, &src_path, &ai_name)?;

    // Generate lib.rs
    let lib_rs_content = write_ai()?;

    let lib_rs_path = format!("{}/lib.rs", src_path);
    let mut lib_rs_file = File::create(lib_rs_path)?;
    lib_rs_file.write_all(lib_rs_content.as_bytes())?;

    Ok(())
}

/// Creates the AI package with a `Cargo.toml` file and the required directory structure.
///
/// # Arguments
/// - `ai_path` - The root path of the AI.
/// - `src_path` - The source directory path.
/// - `ai_name` - The name of the AI package.
///
/// # Returns
/// - `Ok(())` if the package was successfully created.
/// - `Err(io::Error)` if any file or directory operation fails.
fn create_ai_package(ai_path: &str, src_path: &str, ai_name: &str) -> std::io::Result<()>{

    // Create AI directory structure
    fs::create_dir_all(&src_path)?;

    // Generate Cargo.toml
    let cargo_toml_content = format!(
        r#"[package]
    name = "{0}"
    version = "0.1.0"
    edition = "2021"
        
    [dependencies]
    rand = "0.8"
    board = {{ path = "../../board" }}
        
    [lib]
    name = "{0}"
    crate-type = ["cdylib"]
        "#,
        ai_name
    );

    let cargo_toml_path = format!("{}/Cargo.toml", ai_path);
    let mut cargo_toml_file = File::create(cargo_toml_path)?;
    cargo_toml_file.write_all(cargo_toml_content.as_bytes())?;

    println!("📦 Cargo package '{}' créé avec succès.", ai_name);

    Ok(())
}

/// Prompts the user to write AI code interactively.
///
/// # Returns
/// - `Ok(String)` containing the AI source code.
/// - `Err(io::Error)` if reading from standard input fails.
fn write_ai() -> std::io::Result<String> {    
    // Demander le code de l'IA
    println!("✍️  Écrivez votre IA (finissez avec une ligne vide) :");
    let mut code = String::from("use board::Board;\n\n");
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("❌ Échec de la lecture de l'entrée");
        if line.trim().is_empty() {
            break;
        }
        code.push_str(&line);
    }
    
    Ok(code)
}

/// Loads an AI function from a compiled shared library.
///
/// # Arguments
/// - `ai_file` - Path to the compiled AI library file.
///
/// # Returns
/// - `Some((Arc<Library>, Box<dyn Fn(&Board, char) -> Option<((usize, usize), (usize, usize))>>, String))`
///   if the AI function is successfully loaded.
/// - `None` if the library could not be loaded.
fn load_ai_function(ai_file: PathBuf) -> Option<(Arc<Library>, Box<dyn Fn(&Board, char) -> Option<((usize, usize), (usize, usize))> + Send + Sync>, String)> {

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