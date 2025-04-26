// ai_driver/src/main.rs

use board::board::Board;
use libloading::Library;
use serde::{Deserialize, Serialize};
use std::env;
use std::io::{self, Write};

#[derive(Deserialize)]
struct Request {
    board: Board,
    player: char,
}

#[derive(Serialize)]
struct Response {
    // None == no move
    mv: Option<((usize, usize), (usize, usize))>,
}

fn main() -> anyhow::Result<()> {
    // 1) First CLI arg is path to the AI dynamic library
    let lib_path = env::args().nth(1)
        .expect("Usage: ai_driver <path_to_ai_lib>");
    let lib = unsafe { Library::new(&lib_path)? };

    // 2) Lookup the symbol
    let func: libloading::Symbol<
        unsafe extern "C" fn(&Board, char) -> Option<((usize, usize), (usize, usize))>
    > = unsafe { lib.get(b"get_ai_move")? };

    // 3) Read JSON request from stdin
    let req: Request = serde_json::from_reader(io::stdin())?;

    // 4) Call into the library
    eprintln!("[DRIVER] About to call get_ai_move from `{}`", lib_path);
    let mv = unsafe { func(&req.board, req.player) };
    eprintln!("[DRIVER] get_ai_move returned: {:?}", mv);
    // 5) Write JSON response
    let resp = Response { mv };
    serde_json::to_writer(io::stdout(), &resp)?;
    io::stdout().flush().expect("Failed to flush stdout");
    eprintln!("[DRIVER] Done. Exiting now.");
    Ok(())
}
