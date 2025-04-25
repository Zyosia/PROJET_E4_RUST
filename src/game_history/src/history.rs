use std::io::{self, Write, BufRead};
use std::fs::{self, File};

/// Documentation for the History struct, which tracks moves in a game.
#[derive(Debug, Clone)]
pub struct History {
    moves: Vec<(char, (usize, usize), (usize, usize))>, // (Player, From, To)
}

impl History {
    /// Creates a new, empty History instance.
    ///
    /// # Returns
    /// - History: An instance with an empty move list.
    pub fn new() -> Self {
        Self { moves: Vec::new() }
    }

    /// Getter that returns an iterator over the moves.
    ///
    /// # Returns
    /// - An iterator over references to the moves.
    pub fn get_moves(&self) -> impl Iterator<Item = &(char, (usize, usize), (usize, usize))> {
        self.moves.iter()
    }

    /// Records a move in the history.
    ///
    /// # Parameters
    /// - `player`: The player making the move ('X' or 'O').
    /// - `from`: The starting position of the move.
    /// - `to`: The destination position of the move.
    pub fn record_move(&mut self, player: char, from: (usize, usize), to: (usize, usize)) {
        self.moves.push((player, from, to));
    }

    /// Saves the recorded moves to a file.
    ///
    /// # Parameters
    /// - `filename`: The name of the file where the history will be stored.
    ///
    /// # Returns
    /// - `io::Result<()>`: Returns `Ok(())` if successful, otherwise returns an error.
    pub fn save_to_file(&self, filename: &str) -> io::Result<()> {

        let folder_path = "src/games_history";
        let file_path = format!("{}/{}", folder_path, filename);

        // Créer le dossier s'il n'existe pas
        fs::create_dir_all(folder_path)?;

        let mut file = File::create(&file_path)?;
        for (player, from, to) in &self.moves {
            writeln!(file, "{} {} {} {} {}", player, from.0, from.1, to.0, to.1)?;
        }
        println!("✅ Historique de la partie enregistré dans: {}.", file_path);
        Ok(())
    }

    /// Loads game history from a file.
    ///
    /// # Parameters
    /// - `filename`: The name of the file to read from.
    ///
    /// # Returns
    /// - `io::Result<History>`: Returns a `History` instance if successful, otherwise an error.
    pub fn load_from_file(filename: &str) -> io::Result<Self> {
        let file_path = format!("src/games_history/{}", filename);
        let file = File::open(&file_path)?;
        let reader = io::BufReader::new(file);
        let mut history = History::new();

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 5 {
                let player = parts[0].chars().next().unwrap();
                let from = (parts[1].parse().unwrap(), parts[2].parse().unwrap());
                let to = (parts[3].parse().unwrap(), parts[4].parse().unwrap());
                history.moves.push((player, from, to));
            }
        }
        println!("✅ Historique de la partie chargé depuis: {}.", file_path);
        Ok(history)
    }
}