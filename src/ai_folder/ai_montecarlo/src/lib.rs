use board::board::Board;
use rand::seq::SliceRandom;
use rand::Rng;

const SIMULATIONS: usize = 300;

fn get_valid_moves(board: &Board, player: u8) -> Vec<((usize, usize), (usize, usize))> {
    let mut moves = Vec::new();
    for x in 0..8 {
        for y in 0..8 {
            if board.grid[x][y] == player as char {
                let dx = if player == b'X' { 1 } else { -1 };
                for dy in [-1, 0, 1] {
                    let nx = x as isize + dx;
                    let ny = y as isize + dy;
                    if nx >= 0 && nx < 8 && ny >= 0 && ny < 8 {
                        let to = (nx as usize, ny as usize);
                        if board.is_valid_move((x, y), to, player as char) {
                            moves.push(((x, y), to));
                        }
                    }
                }
            }
        }
    }
    moves
}

fn simulate(mut board: Board, player: u8) -> f64 {
    let mut current = player;
    let mut rng = rand::thread_rng();
    while board.check_winner().is_none() {
        let moves = get_valid_moves(&board, current);
        if moves.is_empty() {
            return if current == player { 0.0 } else { 1.0 };
        }
        let m = *moves.choose(&mut rng).unwrap();
        board.apply_move(m.0, m.1);
        current = if current == b'X' { b'O' } else { b'X' };
    }

    match board.check_winner() {
        Some(w) if w == player as char => 1.0,
        Some(_) => 0.0,
        _ => 0.5,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn get_ai_move(board: &Board, player: u8) -> Option<((usize, usize), (usize, usize))> {
    let moves = get_valid_moves(board, player);
    if moves.is_empty() {
        return None;
    }

    let mut best_score = -1.0;
    let mut best_move = None;

    for &m in &moves {
        let mut score = 0.0;
        for _ in 0..SIMULATIONS {
            let mut b = board.clone();
            b.apply_move(m.0, m.1);
            score += simulate(b, player);
        }
        if score > best_score {
            best_score = score;
            best_move = Some(m);
        }
    }

    best_move
}

