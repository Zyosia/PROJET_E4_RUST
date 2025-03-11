use std::fs;
use std::sync::Arc;
use std::process::exit;
use libloading::{Library, Symbol};

extern crate board;
use board::Board;

struct Game {
    board: Board,
    player: char,
    ai_library: Option<Arc<Library>>, // Keep a reference to the library
    ai_function: Option<Box<dyn Fn(&Board, char) -> Option<((usize, usize), (usize, usize))>>>,
}

impl Game {
    fn new(
        ai_library: Option<Arc<Library>>, 
        ai_function: Option<Box<dyn Fn(&Board, char) -> Option<((usize, usize), (usize, usize))>>>
    ) -> Self {
        Self {
            board: Board::new(),
            player: 'X',
            ai_library, // Now properly initialized
            ai_function,
        }
    }
    
    fn run(&mut self) {
        loop {
            self.board.print();

            if let Some(ai_move_function) = &self.ai_function {
                if let Some((from, to)) = ai_move_function(&self.board, self.player) {
                    println!("Joueur {} joue de {:?} à {:?}", self.player, from, to);
                    self.board.apply_move(from, to);
                    if let Some(winner) = self.board.check_winner() {
                        self.board.print();
                        println!("Le joueur {} gagne!", winner);
                        break;
                    }
                    self.player = if self.player == 'X' { 'O' } else { 'X' };
                } else {
                    println!("Aucun coup possible. Le joueur {} perd!", self.player);
                    break;
                }
            } else {
                println!("Erreur : Aucune IA chargée.");
                break;
            }
        }
    }
}

fn load_ai_function() -> Option<(Arc<Library>, Box<dyn Fn(&Board, char) -> Option<((usize, usize), (usize, usize))>>)> {
    let ai_folder = "ai/";

    // Find first AI file
    let ai_file = fs::read_dir(ai_folder).ok()?.find_map(|entry| {
        let entry = entry.ok()?;
        let path = entry.path();
        if path.is_file() && path.extension()? == "so" {
            Some(path)
        } else {
            None
        }
    })?;

    println!("Chargement de l'IA depuis {:?}", ai_file);

    // Load the shared library dynamically
    let lib = Arc::new(unsafe { Library::new(ai_file).ok()? });

    // SAFELY load the function pointer before returning the closure
    let func: Symbol<unsafe fn(&Board, char) -> Option<((usize, usize), (usize, usize))>> =
        unsafe { lib.get(b"get_ai_move").ok()? };

    // Store a raw function pointer to avoid borrowing issues
    let func_ptr = *func;

    // Create a closure that calls the function
    let ai_function = unsafe{move |board: &Board, player: char| func_ptr(board, player)};

    Some((lib, Box::new(ai_function)))
}





fn main() {
    let ai_data = load_ai_function();

    if ai_data.is_none() {
        eprintln!("Aucune IA trouvée dans le dossier 'ai/'. Ajoutez un fichier .so.");
        exit(1);
    }

    let (ai_library, ai_function) = ai_data.unwrap();

    println!("L'IA a été chargée avec succès.");

    let mut game = Game::new(Some(ai_library), Some(ai_function));

    game.run();
}

