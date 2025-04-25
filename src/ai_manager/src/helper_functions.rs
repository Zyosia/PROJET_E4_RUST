use std::{fs,fs::File, path::{PathBuf}, io::{self, Write}};

/// Checks if an AI with the given name already exists in `src/ai_folder/`.
///
/// # Arguments
/// - `ai_name`: The name of the AI package to check.
///
/// # Returns
/// - `true` if the AI already exists.
/// - `false` otherwise.
pub fn ai_exists(ai_name: &str) -> bool {
    let ai_path = PathBuf::from(format!("src/ai_folder/{}", ai_name));
    ai_path.exists() && ai_path.is_dir()
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
pub fn create_ai_package(ai_path: &str, src_path: &str, ai_name: &str) -> std::io::Result<()>{

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
pub fn write_ai() -> std::io::Result<String> {    
    // Demander le code de l'IA
    println!("✍️  Écrivez votre IA (finissez avec une ligne vide) :");
    let mut code = String::from("use board::board::Board;\n\n");
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