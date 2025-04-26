use board::board::Board;

use rand::seq::SliceRandom;  // Ajoute cette importation
#[unsafe(no_mangle)]
pub extern "C" fn get_ai_move(board: &Board, player: char) -> Option<((usize, usize), (usize, usize))> {
    println!("Entering IA 4");
    let mut moves = Vec::new();
    for x in 0..8 {
        for y in 0..8 {
            if board.grid[x][y] == player {
                let dx: isize = if player == 'X' { 1 } else { -1 };
                let possible_moves = vec![
                    ((x as isize + dx) as usize, y),  // Convertir en isize pour l'opération, puis revenir en usize
                    ((x as isize + dx) as usize, y.saturating_sub(1)),  
                    ((x as isize + dx) as usize, y.saturating_add(1)),  
                ];                
                for &to in &possible_moves {
                    if to.0 < 8 && to.1 < 8 && board.is_valid_move((x, y), to, player) {
                        moves.push(((x, y), to));
                    }
                }
            }
        }
    }
    // Choisir un coup aléatoire parmi les coups valides
    let mut rng = rand::thread_rng();
    moves.choose(&mut rng).cloned()
}
