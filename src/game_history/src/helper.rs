use std::{io, fs};

/// Lists all the available games stored in the game history directory.
/// Returns a vector of game file names if successful, or an error if reading the directory fails.
pub fn list_available_games() -> io::Result<Vec<String>> {
    let path = "src/games_history";
    let entries = fs::read_dir(path)?;
    let mut games = Vec::new();

    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name().into_string().unwrap();
        if file_name.ends_with(".txt") && !file_name.starts_with("replay"){
            games.push(file_name);
        }
    }

    Ok(games)
}

/// Deletes all the game history files (with a `.txt` extension) in the game history directory.
/// Returns an `io::Result` indicating success or failure.
pub fn delete_all_history_files() -> io::Result<()> {
    let path = "src/games_history";

    // Read the directory contents
    let entries = fs::read_dir(path)?;

    // Iterate over the entries and delete files that end with .txt
    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name();
        
        if file_name.to_string_lossy().ends_with(".txt") {
            let file_path = entry.path();
            println!("Suppression: {}", file_path.display());
            fs::remove_file(file_path)?;
        }
    }

    println!("✅ Suppression de l'historique terminée.");
    Ok(())
}