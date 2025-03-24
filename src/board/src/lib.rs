use std::{io, io::Write};

const SIZE: usize = 8;

#[derive(Clone)]
pub struct Board {
    pub grid: [[char; SIZE]; SIZE],
}

impl Board {
    pub fn new() -> Self {
        let mut grid = [['.'; SIZE]; SIZE];
        for i in 0..2 {
            for j in 0..SIZE {
                grid[i][j] = 'X';
            }
        }
        for i in SIZE - 2..SIZE {
            for j in 0..SIZE {
                grid[i][j] = 'O';
            }
        }
        Self { grid }
    }
    
    pub fn print(&self) {
        println!("  A B C D E F G H");
        for (i, row) in self.grid.iter().rev().enumerate() {
            print!("{} ", SIZE - i);
            for cell in row.iter() {
                print!("{} ", cell);
            }
            println!("{}", SIZE - i);
        }
        println!("  A B C D E F G H\n");
    }
    
    pub fn to_string(&self) -> String {
        let mut board_str = String::from("  A B C D E F G H\n");

        // Iterate through the rows, reversing for the correct row order
        for (i, row) in self.grid.iter().rev().enumerate() {
            board_str.push_str(&format!("{} ", SIZE - i)); // Add row number
            for cell in row.iter() {
                board_str.push_str(&format!("{} ", cell)); // Add cell content
            }
            board_str.push_str(&format!("{}\n", SIZE - i)); // Add row number at the end
        }

        board_str.push_str("  A B C D E F G H\n"); // Add column labels at the bottom
        board_str
    }

    pub fn print_to_file<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        let board_output = self.to_string();
        writeln!(writer, "{}", board_output)?;
        Ok(())
    }

    pub fn is_valid_move(&self, from: (usize, usize), to: (usize, usize), player: char) -> bool {
        let (fx, fy) = from;
        let (tx, ty) = to;
        if tx >= SIZE || ty >= SIZE || self.grid[fx][fy] != player || self.grid[tx][ty] == player {
            return false;
        }
        let direction = if player == 'X' { 1 } else { -1 };
        let forward_moves = vec![
            (fx as isize + direction, fy as isize),
            (fx as isize + direction, fy as isize - 1),
            (fx as isize + direction, fy as isize + 1),
        ];
        let capture_moves = vec![
            (fx as isize + direction, fy as isize - 1),
            (fx as isize + direction, fy as isize + 1),
        ];
        if forward_moves.contains(&(tx as isize, ty as isize)) && self.grid[tx][ty] == '.' {
            return true;
        }
        if capture_moves.contains(&(tx as isize, ty as isize)) && self.grid[tx][ty] != '.' && self.grid[tx][ty] != player {
            return true;
        }
        false
    }
    
    pub fn apply_move(&mut self, from: (usize, usize), to: (usize, usize)) {
        let (fx, fy) = from;
        let (tx, ty) = to;
        self.grid[tx][ty] = self.grid[fx][fy];
        self.grid[fx][fy] = '.';
    }
    
    pub fn check_winner(&self) -> Option<char> {
        let mut x_count = 0;
        let mut o_count = 0;
        for row in self.grid.iter() {
            for &cell in row.iter() {
                if cell == 'X' {
                    x_count += 1;
                } else if cell == 'O' {
                    o_count += 1;
                }
            }
        }
        if x_count == 0 {
            return Some('O');
        }
        if o_count == 0 {
            return Some('X');
        }
        for j in 0..SIZE {
            if self.grid[SIZE - 1][j] == 'X' {
                return Some('X');
            }
            if self.grid[0][j] == 'O' {
                return Some('O');
            }
        }
        None
    }
}