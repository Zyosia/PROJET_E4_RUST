use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};
use std::{io::{self, Write}, fs};
use tournament_engine::{Tournament, TournamentState};
use game_engine::replay_from_file;

fn main() {
    loop {
        match main_menu() {
            1 => start_tournament(),
            2 => {
                println!("👋 Exiting...");
                break;
            }
            _ => {
                println!("❌ Invalid choice. Try again.");
                pause();
            }
        }
    }
}

fn main_menu() -> u32 {
    clear_screen();
    println!("🎮 Welcome to the Tournament Manager 🎮");
    println!("1️⃣  Create a Tournament");
    println!("2️⃣  Exit");
    print!("👉 Enter your choice: ");
    read_input()
}

fn start_tournament() {
    let mut tournament = Tournament::new();

    loop {
        match tournament_menu(&tournament) {
            1 => {
                clear_screen();
                if let Err(e) = tournament.add_ia() {
                    println!("{}", e);
                }
                pause();
            }
            2 => {
                clear_screen();
                tournament.show_participants();
                pause();
            }
            3 => {
                clear_screen();
                if let Err(e) = tournament.start() {
                    println!("{}", e);
                }
                pause();
            }
            4 => {
                clear_screen();
                tournament.display_scoreboard();
                pause();
            }
            5 => {
                clear_screen();
                if let Err(e) = tournament.run() {
                    println!("{}", e);
                }
                pause();
            }
            6 => {
                clear_screen();
                replay_game(); // Handle the replay game option
            }
            7 => {
                clear_screen();
                println!("🏁 Ending Tournament...");
                // Delete all history files
                if let Err(e) = delete_all_history_files() {
                    println!("❌ Error deleting history files: {}", e);
                }
                pause();
                return; // Go back to the main menu
            }
            _ => {
                println!("❌ Invalid choice. Try again.");
                pause();
            }
        }
    }
}

fn tournament_menu(tournament: &Tournament) -> u32 {
    clear_screen();
    
    // Get the tournament state
    let state = match tournament.get_state() {
        TournamentState::Waiting => "⏳ Waiting",
        TournamentState::Started => "✅ Started",
    };

    println!("🏆 Tournament Menu 🏆");
    println!("📌 Status: {}", state);
    println!("1️⃣  Add AI");
    println!("2️⃣  Show Participants");
    println!("3️⃣  Start Tournament");
    println!("4️⃣  Display Scoreboard");
    println!("5️⃣  Run Tournament");
    println!("6️⃣  Replay Game");
    println!("7️⃣  End Tournament");
    print!("👉 Enter your choice: ");
    read_input()
}

fn replay_game() {
    match list_available_games() {
        Ok(games) => {
            if games.is_empty() {
                println!("❌ No games found in history.");
                pause();
                return;
            }

            println!("🎮 Select a game to replay:");

            for (index, game) in games.iter().enumerate() {
                println!("{}: {}", index + 1, game);
            }

            print!("👉 Enter the number of the game to replay: ");
            let choice = read_input();

            if choice > 0 && choice <= games.len() as u32 {
                let selected_game = &games[(choice - 1) as usize];
                let parts: Vec<&str> = selected_game.split("_vs_").collect();
                let ai_name1 = parts[0];
                let ai_name2 = parts[1].trim_end_matches(".txt");

                // Replay the selected game
                if let Err(e) = replay_from_file(ai_name1, ai_name2) {
                    println!("❌ Error replaying the game: {}", e);
                }
                pause();
            } else {
                println!("❌ Invalid selection.");
                pause();
            }
        }
        Err(e) => {
            println!("❌ Error reading game history: {}", e);
            pause();
        }
    }
}

fn clear_screen() {
    execute!(io::stdout(), Clear(ClearType::All)).unwrap();
}

fn read_input() -> u32 {
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap_or(0)
}

// ⏸️ Pause function: Wait for user to press Enter
fn pause() {
    println!("🔄 Press Enter to continue...");
    let mut pause_input = String::new();
    io::stdin().read_line(&mut pause_input).unwrap();
}

fn list_available_games() -> io::Result<Vec<String>> {
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

fn delete_all_history_files() -> io::Result<()> {
    let path = "src/games_history";

    // Read the directory contents
    let entries = fs::read_dir(path)?;

    // Iterate over the entries and delete files that end with .txt
    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name();
        
        if file_name.to_string_lossy().ends_with(".txt") {
            let file_path = entry.path();
            println!("Deleting file: {}", file_path.display());
            fs::remove_file(file_path)?;
        }
    }

    println!("✅ All history files deleted.");
    Ok(())
}