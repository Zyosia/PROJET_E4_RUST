use board::board::Board;

/// 🚨 WARNING: This AI never returns — it's meant for timeout testing only.
#[no_mangle]
pub extern "C" fn get_ai_move(_board: &Board, _player: char) -> Option<((usize, usize), (usize, usize))> {
    loop {
        // Infinite loop to simulate a hanging AI
        std::thread::sleep(std::time::Duration::from_secs(2));
    }
    
    // This is unreachable, but required for return type completeness
    #[allow(unreachable_code)]
    None
}
