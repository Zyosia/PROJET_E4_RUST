use std::{fs::{self, File}, path::Path, io::{self, Write}};
use crate::tournament::Tournament;

/// Displays the tournament scoreboard with rankings.
pub fn display_scoreboard(tournament: &Tournament) {
    let scoreboard = tournament.get_all_scores();
    let mut sorted_scores: Vec<(&String, &usize)> = scoreboard.iter().collect();
    sorted_scores.sort_by(|a, b| b.1.cmp(a.1)); // Sort descending

    println!("\n🏆 Résultats du Tournoi 🏆");
    for (rank, (ai_name, score)) in sorted_scores.iter().enumerate() {
        println!("{}. {} - {} points", rank + 1, ai_name, score);
    }
}

/// Displays the list of AI participants in the tournament.
pub fn show_participants(tournament: &Tournament) {
    if tournament.get_participants().is_empty() {
        println!("❌ Aucun participant enregistré.");
        return;
    }
    println!("\n🤖 Participants du tournoi:");
    for (i, ai) in tournament.get_participants().iter().enumerate() {
        println!("{}. {}", i + 1, ai);
    }
}

pub fn save_tournament_to_file(tournament: &Tournament, filename: &str) -> io::Result<()> {
    let mut file = File::create(filename)?;

    // Writing the tournament data to the file
    writeln!(file, "Tournament Name: {}", tournament.get_name())?;
    writeln!(file, "Tournament State: {}", tournament.get_state_to_string())?;
    writeln!(file, "Participants:")?;
    for participant in &tournament.get_participants() {
        writeln!(file, "    {}", participant)?;
    }
    writeln!(file, "AI Library Paths:")?;
    for (name, path) in &tournament.get_ai_libs() {
        writeln!(file, "    {} -> {}", name, path)?;
    }
    writeln!(file, "Scores:")?;
    for (name, score) in &tournament.get_all_scores() {
        writeln!(file, "    {} -> {}", name, score)?;
    }
    writeln!(file, "Number of Games: {}", tournament.get_number_of_games())?;

    Ok(())
}

/// Save all `.txt` files from `games_history` to `tournament_save`
pub fn save_tournament(tournament: &Tournament) -> io::Result<()> {
    let source_folder = "src/games_history";
    let destination_folder = format!("src/.tournament_saves/{}", tournament.get_name());
    let games_history_destination_folder = format!("src/.tournament_saves/{}/tournament_data", tournament.get_name());
    let tournament_data_destination_folder = format!("src/.tournament_saves/{}/tournament_games_history", tournament.get_name());

    // Ensure all the destinations folders exists
    if !Path::new(&destination_folder).exists() {
        fs::create_dir(&destination_folder)?;
    }
    if !Path::new(&games_history_destination_folder).exists() {
        fs::create_dir(&games_history_destination_folder)?;
    }
    if !Path::new(&tournament_data_destination_folder).exists() {
        fs::create_dir(&tournament_data_destination_folder)?;
    }

    // Read all entries in the source folder
    for entry in fs::read_dir(source_folder)? {
        let entry = entry?;
        let path = entry.path();
        // Check if the entry is a file and ends with .txt
        if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("txt") {
            let file_name = path.file_name().unwrap(); // Get the file name
            let new_path = Path::new(&games_history_destination_folder).join(file_name); // Destination path

            // Move the file
            fs::rename(&path, &new_path)?;

            println!("✅ {} saved.", file_name.to_string_lossy());
        }
    }

    let tournament_data_file_name = format!("{}/data.txt", tournament_data_destination_folder);
    save_tournament_to_file(tournament, &tournament_data_file_name).unwrap();

    println!("✅ {} saved", tournament.get_name());
    Ok(())
}