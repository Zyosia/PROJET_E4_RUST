use board::Board;
use std::sync::Arc;
use libloading::{Library};

pub struct Game {  
    board: Board,
    player: char,
    lib_ai1: Option<Arc<Library>>,
    ai_function1: Option<Box<dyn Fn(&Board, char) -> Option<((usize, usize), (usize, usize))>>>,
    lib_ai2: Option<Arc<Library>>,
    ai_function2: Option<Box<dyn Fn(&Board, char) -> Option<((usize, usize), (usize, usize))>>>,
}

impl Game { // Implement Game with the same lifetime 'a
    pub fn new(
        lib_ai1: Option<Arc<Library>>,
        ai_function1: Option<Box<dyn Fn(&Board, char) -> Option<((usize, usize), (usize, usize))>>>,
        lib_ai2: Option<Arc<Library>>,
        ai_function2: Option<Box<dyn Fn(&Board, char) -> Option<((usize, usize), (usize, usize))>>>
    ) -> Self {
        Self {
            board: Board::new(),
            player: 'X',
            lib_ai1,
            ai_function1,
            lib_ai2,
            ai_function2,
        }
    }
    
    /* One AI run
    pub fn run(&mut self) {
        loop {
            self.board.print();
            
            print_type_of(&self.ai_function1);
            print_type_of(&self.board);
            print_type_of(&self.player);

            if let Some(ai_move_function) = &self.ai_function1 {
                println!("Trying to use ai");
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
    */

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
            
                // Pass the board reference directly without unsafe
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