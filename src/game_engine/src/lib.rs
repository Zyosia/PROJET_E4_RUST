use board::Board;
use std::sync::Arc;
use libloading::{Library};

#[allow(dead_code)]
pub struct Game {  
    board: Board,
    player: char,
    ai_function1: Option<Arc<Box<dyn Fn(&Board, char) -> Option<((usize, usize), (usize, usize))>>>>,
    ai_function2: Option<Arc<Box<dyn Fn(&Board, char) -> Option<((usize, usize), (usize, usize))>>>>,
    lib_ai1: Option<Arc<Library>>,
    lib_ai2: Option<Arc<Library>>,
}

impl Game {
    pub fn new(
        ai_function1: Option<Arc<Box<dyn Fn(&Board, char) -> Option<((usize, usize), (usize, usize))>>>>,
        ai_function2: Option<Arc<Box<dyn Fn(&Board, char) -> Option<((usize, usize), (usize, usize))>>>>,
        lib_ai1: Option<Arc<Library>>,
        lib_ai2: Option<Arc<Library>>
    ) -> Self {
        Self {
            board: Board::new(),
            player: 'X',
            ai_function1,
            ai_function2,
            lib_ai1,
            lib_ai2,
        }
    }

    pub fn get_board(&mut self) -> Board {
        self.board.clone()
    }

    /*Two AI Run*/
    pub fn run(&mut self) {
        loop {
            self.board.print();
            
            let ai_move_function = if self.player == 'X' {
                self.ai_function1.as_ref()
            } else {
                self.ai_function2.as_ref()
            };
            
            if let Some(ai_move_function) = ai_move_function {
                println!("Attempting to execute AI function for player {}", self.player);
            
                match ai_move_function(&self.board, self.player) {
                    Some((from, to)) => {
                        println!("Joueur {} joue de {:?} à {:?}", self.player, from, to);
                        self.board.apply_move(from, to);
            
                        if let Some(winner) = self.board.check_winner() {
                            self.board.print();
                            println!("Le joueur {} gagne!", winner);
                            break;
                        }
                        self.player = if self.player == 'X' { 'O' } else { 'X' };
            
                    }
                    None => {
                        println!("Aucun coup possible. Le joueur {} perd!", self.player);
                        break;
                    }
                }
            } else {
                println!("Erreur : Aucune IA chargée pour le joueur {}.", self.player);
                break;
            }
        }
    }
}