use std::{io, time::Duration, thread::sleep};
use crate::history::History;
use board::{board::Board, helper::format_position};

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
        let (from_row, from_col) = format_position(*from);
        let (to_row, to_col) = format_position(*to);

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