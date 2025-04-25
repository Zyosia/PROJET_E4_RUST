use std::{fs,fs::File, path::{PathBuf}, io::{self, Write}};
use crate::helper_functions::*;

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

/// Creates a new AI by copying source code from a given file path.
///
/// This function creates the AI package structure, including the `Cargo.toml`,
/// and then copies the provided file into the new AI's `lib.rs`.
///
/// # Arguments
/// - `ai_name`: Name of the AI package to create.
/// - `source_file_path`: Path to the file containing the AI source code.
///
/// # Returns
/// - `Ok(())` if the AI package is created successfully.
/// - `Err(io::Error)` if any file operation fails.
pub fn create_ai_from_file(ai_name: &str, source_file_path: &PathBuf) -> std::io::Result<()> {
    if ai_name.trim().is_empty() {
        eprintln!("❌ Le nom de l'IA ne peut pas être vide.");
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Nom d'IA vide"));
    }

    if !source_file_path.exists() || !source_file_path.is_file() {
        eprintln!("❌ Le fichier source spécifié est invalide.");
        return Err(io::Error::new(io::ErrorKind::NotFound, "Fichier source introuvable"));
    }
    
    let ai_path = format!("src/ai_folder/{}", ai_name);
    let src_path = format!("{}/src", ai_path);

    // Create the package structure
    create_ai_package(&ai_path, &src_path, ai_name)?;

    // Copy the AI source file into lib.rs
    let lib_rs_path = format!("{}/lib.rs", src_path);
    fs::copy(source_file_path, lib_rs_path)?;

    println!("📄 Code AI copié depuis {:?} vers le nouveau package '{}'", source_file_path, ai_name);

    Ok(())
}