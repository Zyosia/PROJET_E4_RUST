
/// Formats a board position from `(usize, usize)` to `(char, usize)`.
///
/// # Parameters
/// - `position`: A tuple `(row, column)` representing the board position.
///
/// # Returns
/// - `(char, usize)`: A tuple where the row is converted to a letter and column is 1-based.
pub fn format_position(position: (usize, usize)) -> (char, usize) {
    let row_char = (b'A' + position.1 as u8) as char; // Convert row index to letter
    let col_number = position.0 + 1;
    (row_char, col_number)
}