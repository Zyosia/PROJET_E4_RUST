use board::{board::Board, helper::format_position};
use crate::helper::*;
use game_history::history::History;
use std::io::{Write, BufWriter};
use std::fs::{self, File};
use std::time::{Duration};

/// Represents a game instance, managing the board, players, and AI interactions.
#[allow(dead_code)]
pub struct Game {
    board: Board,
    player: char,
    history: History,
    ai_name1: String,
    ai_name2: String,
    ai1_lib_path: String,
    ai2_lib_path: String,
    winner: char,
}

/// Creates a new game instance with AI functions and libraries.
///
/// # Parameters
/// - ai_name1: Name of AI 1.
/// - ai_name2: Name of AI 2.
/// - ai_function1: Optional AI function for player X.
/// - ai_function2: Optional AI function for player O.
/// - lib_ai1: Optional dynamic library for AI 1.
/// - lib_ai2: Optional dynamic library for AI 2.
///
/// # Returns
/// - Game: A new game instance.
impl Game {
    pub fn new(
        ai_name1: String,
        ai_name2: String,
        ai1_lib_path: String,
        ai2_lib_path: String,
    ) -> Self {
        Self {
            board: Board::new(),
            player: 'X',
            history: History::new(),
            ai_name1,
            ai_name2,
            ai1_lib_path,
            ai2_lib_path,
            winner: ' ',
        }
    }

    /// Returns a copy of the current board state.
    /// # Returns
    /// - `Board`: A Board object representing the board.
    pub fn get_board(&self) -> Board {
        self.board.clone()
    }

    pub fn get_winner(&self) -> char {
        self.winner.clone()
    }

    fn set_winner(&mut self, p_winner: char){
        self.winner = p_winner.clone();
    }

    /// Runs the game loop, executing AI moves until a winner is determined or no moves are possible.
    ///
    /// # Returns
    /// - `Result<(), Box<dyn std::error::Error>>`: Returns `Ok(())` if the game completes successfully, otherwise an error.
    pub fn run(&mut self, game_number: i32) -> Result<(), Box<dyn std::error::Error>> {
        let history_filename = format!("{}_vs_{}_game_{}.txt", self.ai_name1, self.ai_name2, game_number);
        let folder_path = "src/games_history";
        let replay_filename = format!("{}/replay_{}_vs_{}_game_{}.txt", folder_path, self.ai_name1, self.ai_name2, game_number);
        
        // Créer le dossier s'il n'existe pas
        fs::create_dir_all(folder_path)?;

        // Open the replay file for writing
        let file = File::create(&replay_filename)?;
        let mut writer = BufWriter::new(file);

        // Function to write to file
        let mut file_writer = &mut writer;

        loop {
            //self.board.print();
            
            let ai_lib_path = if self.player == 'X' {
                self.ai1_lib_path.clone()
            } else {
                self.ai2_lib_path.clone()
            };
            
            if !ai_lib_path.is_empty() {

                let result = get_ai_move_with_process_timeout(
                    &ai_lib_path,  // e.g. "plugins/random_ai.so"
                    &self.board,
                    self.player,
                    Duration::from_secs(1),
                );
                
                match result {
                    Some((from, to)) => {
                        if self.board.is_valid_move(from, to, self.player) {
                            let (from_row, from_col) = format_position(from);
                            let (to_row, to_col) = format_position(to);

                            println!("Joueur {} joue de ({},{}) à ({},{})", self.player, from_row, from_col, to_row, to_col);

                            let output = format!("🔁 Replay: Joueur {} joue de ({},{}) à ({},{})", self.player, from_row, from_col, to_row, to_col);
                            writeln!(file_writer, "{}", output)?;
                            self.board.apply_move(from, to);
                            self.history.record_move(self.player, from, to);
                            writeln!(file_writer, "Plateau après le coup:")?;
                            self.board.print_to_file(&mut file_writer)?;

                            if let Some(winner) = self.board.check_winner() {
                                self.board.print();
                                println!("🏆 Le joueur {} gagne!", winner);
                                let output = format!("🏆 Le joueur {} gagne!", winner);
                                writeln!(file_writer, "{}", output)?;
                                self.history.save_to_file(&history_filename).unwrap();
                                println!("✅ Replay enregistré dans: {}", replay_filename);
                                self.set_winner(winner);
                                break Ok (());
                            }
                            self.player = if self.player == 'X' { 'O' } else { 'X' };
                        } else {
                            let (from_row, from_col) = format_position(from);
                            let (to_row, to_col) = format_position(to);
                            let output = format!("🔁 Replay: Joueur {} joue de ({},{}) à ({},{})", self.player, from_row, from_col, to_row, to_col);
                            writeln!(file_writer, "{}", output)?;
                            let output = format!("❌ Coup illégal. Le joueur {} perd!", self.player);
                            println!("{}",output);
                            writeln!(file_writer, "{}", output)?;
                            self.history.save_to_file(&history_filename).unwrap();
                            println!("✅ Replay enregistré dans: {}", replay_filename);
                            let winner = if self.player == 'X' { 'O' } else { 'X' };
                            self.set_winner(winner);
                            break Ok(());
                        }
                        
                    }
                    None => {
                        let output = format!("⏰ Temps écoulé ou IA n'a retourné aucun coup. Le joueur {} perd!", self.player);
                        println!("{}",output);
                        writeln!(file_writer, "{}", output)?;
                        self.history.save_to_file(&history_filename).unwrap();
                        println!("✅ Replay enregistré dans: {}", replay_filename);
                        let winner = if self.player == 'X' { 'O' } else { 'X' };
                        self.set_winner(winner);
                        break Ok(());
                    }
                }
            } else {
                let err_mess = format!("❌ Erreur : Aucune IA chargée pour le joueur {}.", self.player);
                self.history.save_to_file(&history_filename).unwrap();
                break Err(err_mess.to_string().into());
            } //else
        } //loop
    } // fn run
}