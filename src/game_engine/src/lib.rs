use board::Board;
use game_history::History;
use std::io::{self, Write, BufWriter};
use std::{time::Duration, thread::sleep};
use std::fs::File;
use std::sync::Arc;
use libloading::Library;

/// Represents a game instance, managing the board, players, and AI interactions.
#[allow(dead_code)]
pub struct Game {
    board: Board,
    player: char,
    history: History,
    ai_name1: String,
    ai_name2: String,
    ai_function1: Option<Arc<Box<dyn Fn(&Board, char) -> Option<((usize, usize), (usize, usize))>>>>,
    ai_function2: Option<Arc<Box<dyn Fn(&Board, char) -> Option<((usize, usize), (usize, usize))>>>>,
    lib_ai1: Option<Arc<Library>>,
    lib_ai2: Option<Arc<Library>>,
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
        ai_function1: Option<Arc<Box<dyn Fn(&Board, char) -> Option<((usize, usize), (usize, usize))>>>>,
        ai_function2: Option<Arc<Box<dyn Fn(&Board, char) -> Option<((usize, usize), (usize, usize))>>>>,
        lib_ai1: Option<Arc<Library>>,
        lib_ai2: Option<Arc<Library>>
    ) -> Self {
        Self {
            board: Board::new(),
            player: 'X',
            history: History::new(),
            ai_name1,
            ai_name2,
            ai_function1,
            ai_function2,
            lib_ai1,
            lib_ai2,
        }
    }

    /// Returns a copy of the current board state.
    /// # Returns
    /// - `Board`: A Board object representing the board.
    pub fn get_board(&self) -> Board {
        self.board.clone()
    }

    /// Formats a board position from `(usize, usize)` to `(char, usize)`.
    ///
    /// # Parameters
    /// - `position`: A tuple `(row, column)` representing the board position.
    ///
    /// # Returns
    /// - `(char, usize)`: A tuple where the row is converted to a letter and column is 1-based.
    fn format_position(position: (usize, usize)) -> (char, usize) {
        let row_char = (b'A' + position.1 as u8) as char; // Convert row index to letter
        let col_number = position.0 + 1;
        (row_char, col_number)
    }

    /// Runs the game loop, executing AI moves until a winner is determined or no moves are possible.
    ///
    /// # Returns
    /// - `Result<(), Box<dyn std::error::Error>>`: Returns `Ok(())` if the game completes successfully, otherwise an error.
    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let history_filename = format!("{}_vs_{}.txt", self.ai_name1, self.ai_name2);
        let replay_filename = format!("src/games_history/replay_{}_vs_{}.txt", self.ai_name1, self.ai_name2);
        
        // Open the replay file for writing
        let file = File::create(&replay_filename)?;
        let mut writer = BufWriter::new(file);

        // Function to write to file
        let mut file_writer = &mut writer;

        loop {
            self.board.print();
            
            let ai_move_function = if self.player == 'X' {
                self.ai_function1.as_ref()
            } else {
                self.ai_function2.as_ref()
            };
            
            if let Some(ai_move_function) = ai_move_function {
                match ai_move_function(&self.board, self.player) {
                    Some((from, to)) => {
                        let (from_row, from_col) = Self::format_position(from);
                        let (to_row, to_col) = Self::format_position(to);

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
                            break Ok (());
                        }
                        self.player = if self.player == 'X' { 'O' } else { 'X' };
                    }
                    None => {
                        println!("❌ Aucun coup possible. Le joueur {} perd!", self.player);
                        self.history.save_to_file(&history_filename).unwrap();
                        println!("✅ Replay enregistré dans: {}", replay_filename);
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

/* History Replayer Functions */

/// Replays a saved game from history.
///
/// # Parameters
/// - ai_name1: The name of AI 1.
/// - ai_name2: The name of AI 2.
///
/// # Returns
/// - io::Result<()>: Returns Ok(()) if successful, otherwise an error.
pub fn replay_from_file(ai_name1: &str, ai_name2: &str) -> io::Result<()> {
    let history_filename = format!("{}_vs_{}.txt", ai_name1, ai_name2);
    let history = History::load_from_file(&history_filename)?;
    let mut board = Board::new();

    history.get_moves().for_each(|(player, from, to)| {
        let (from_row, from_col) = Game::format_position(*from);
        let (to_row, to_col) = Game::format_position(*to);

        println!(
            "🔁 Replay: Joueur {} joue de ({},{}) à ({},{})",
            player, from_row, from_col, to_row, to_col
        );

        board.apply_move(*from, *to);
        board.print();
        sleep(Duration::from_millis(500)); // half second between moves
    });

    Ok(())
}
