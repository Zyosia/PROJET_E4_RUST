use std::{fs,fs::File, sync::Arc, path::{Path,PathBuf}, process::exit, io::{self, Write}};
use libloading::{Library, Symbol};
use board::Board;

pub fn ajouter_ia() {
    // Demander le nom de l'IA
    println!("Entrez le nom de votre IA :");
    let mut ia_name = String::new();
    io::stdin().read_line(&mut ia_name).expect("Échec de la lecture de l'entrée");
    let ia_name = ia_name.trim();
    
    // Créer le fichier correspondant
    let file_name = format!("AI_FOLDER/{}.rs", ia_name);
    let path = Path::new(&file_name);
    let mut file = File::create(&path).expect("Échec de la création du fichier");
    
    // Demander le code de l'IA
    println!("Entrez le code de votre IA (finissez avec une ligne vide) :");
    let mut code = String::new();
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Échec de la lecture de l'entrée");
        if line.trim().is_empty() {
            break;
        }
        code.push_str(&line);
    }
    
    // Écrire le code dans le fichier
    file.write_all(code.as_bytes()).expect("Échec de l'écriture dans le fichier");
    
    println!("Le fichier '{}' a été créé avec succès.", file_name);
}

fn load_ai_function(ai_file: PathBuf) -> Option<(Arc<Library>, Box<dyn Fn(&Board, char) -> Option<((usize, usize), (usize, usize))>>)> {

    println!("Chargement de l'IA depuis {:?}", ai_file);

    // Load the shared library dynamically
    let lib = Arc::new(unsafe { Library::new(&ai_file).ok()? });

    // SAFELY load the function pointer before returning the closure
    let func: Symbol<unsafe fn(&Board, char) -> Option<((usize, usize), (usize, usize))>> =
        unsafe { lib.get(b"get_ai_move").ok()? };

    // Store a raw function pointer to avoid borrowing issues
    let func_ptr = *func;

    // Create a closure that calls the function
    let ai_function = unsafe{move |board: &Board, player: char| func_ptr(board, player)};

    Some((lib, Box::new(ai_function)))
}

pub fn load_ai_functions() -> Option<Vec<Option<(Arc<Library>, Box<dyn Fn(&Board, char) -> Option<((usize, usize), (usize, usize))>>)>>> {
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
                                    eprintln!("Erreur Lecture Data IA");
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
    Some(ai_functions_data) // No need for cloning in the closure
}

/*
pub fn load_ai_functions() -> Option<(Vec<Arc<Library>>, Vec<Box<dyn Fn(&Board, char) -> Option<((usize, usize), (usize, usize))>>>)> {
    let ai_folder = "src/ai_folder/";
    let mut ai_functions = Vec::new();
    let mut libraries = Vec::new(); // Store Arc<Library> to keep them alive

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
                                println!("Chargement de l'IA depuis {:?}", file_path);

                                unsafe {
                                    // Load the shared library
                                    let lib = Arc::new(Library::new(&file_path).ok()?);
                                    libraries.push(lib); // Store the library to keep it alive
                                    // Load the function
                                    let func: Symbol<extern "C" fn(&Board, char) -> Option<((usize, usize), (usize, usize))>> =
                                        libraries.last().unwrap().get(b"get_ai_move").ok()?;

                                    // Explicitly cast the closure to Box<dyn Fn(...)>
                                    let ai_function: Box<dyn Fn(&Board, char) -> Option<((usize, usize), (usize, usize))>> =
                                        Box::new(move |board, player| func(board, player));

                                    ai_functions.push(ai_function);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Some((libraries, ai_functions)) // Now safe: AI functions hold Arc<Library>, preventing early drop
}
*/


/*
pub fn load_ai_functions() -> Option<Vec<Box<dyn Fn(&Board, char) -> Option<((usize, usize), (usize, usize))>>>> {
    let ai_folder = "src/ai_folder/";
    let mut ai_functions = Vec::new();

    // Iterate through all subdirectories in ai_folder
    if let Ok(entries) = fs::read_dir(ai_folder) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let target_path = path.join("target/release");

                // Find all dynamic libraries in the target/release folder
                if let Ok(files) = fs::read_dir(&target_path) {
                    for file in files.flatten() {
                        let file_path = file.path();
                        if file_path.is_file() {
                            let ext = file_path.extension().and_then(|e| e.to_str());

                            // Check for the appropriate file extension based on the OS
                            #[cfg(target_os = "windows")]
                            let is_dynamic_lib = ext == Some("dll");

                            #[cfg(target_os = "linux")]
                            let is_dynamic_lib = ext == Some("so");

                            #[cfg(target_os = "macos")]
                            let is_dynamic_lib = ext == Some("dylib");

                            if is_dynamic_lib {
                                println!("Chargement de l'IA depuis {:?}", file_path);

                                let lib = Library::new(file_path).unwrap();
                                let func: Symbol<extern "C" fn(&Board, char) -> Option<((usize, usize), (usize, usize))>> =
                                    lib.get(b"get_ai_move").unwrap();
                                // Store a raw function pointer to avoid borrowing issues
                                let func_ptr = *func;
                                // Create a closure that calls the function
                                let ai_function = unsafe{move |board: &Board, player: char| func_ptr(board, player)};
                                ai_functions.push(Box::new(ai_function));
                            }
                        }
                    }
                }
            }
        }
    }

    Some(ai_functions)
}
*/