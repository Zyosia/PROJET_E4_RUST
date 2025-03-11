extern crate board;
use board::Board;

#[no_mangle]
pub extern "C" fn get_ai_move(board: &Board, player: char) -> Option<((usize, usize), (usize, usize))> {
    let mut moves = Vec::new();
    for x in 0..8 {
        for y in 0..8 {
            if board.grid[x][y] == player {
                let possible_moves = vec![
                    (x + if player == 'X' { 1 } else { 7 }, y),
                    (x + if player == 'X' { 1 } else { 7 }, y - 1),
                    (x + if player == 'X' { 1 } else { 7 }, y + 1),
                ];
                for &to in &possible_moves {
                    if to.0 < 8 && to.1 < 8 && board.is_valid_move((x, y), to, player) {
                        moves.push(((x, y), to));
                    }
                }
            }
        }
    }
    moves.into_iter().next()
}
