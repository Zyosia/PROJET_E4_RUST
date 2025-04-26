use std::process::{Command, Stdio};
use wait_timeout::ChildExt;
use serde_json::json;
use board::board::Board;
use std::time::Duration;


pub fn get_ai_move_with_process_timeout(
    ai_lib_path: &str,
    board: &Board,
    player: char,
    timeout: Duration,
) -> Option<((usize, usize), (usize, usize))> {
    let driver_path = if cfg!(windows) {
        "src/ai_driver/target/release/ai_driver.exe"
    } else {
        "src/ai_driver/target/release/ai_driver"
    };

    // Prepare the JSON request
    let req = json!({ "board": board, "player": player }).to_string();

    // Start the child process
    let mut child = Command::new(driver_path)
        .arg(ai_lib_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to spawn ai_driver");

    // Write to the child's stdin
    if let Some(mut stdin) = child.stdin.take() {
        use std::io::Write;
        stdin.write_all(req.as_bytes()).expect("Failed to write to stdin");
    }

    // Wait for the child process with a timeout
    match child.wait_timeout(timeout).expect("Failed to wait on child") {
        Some(status) if status.success() => {
            // Read the output
            let output = child
                .wait_with_output()
                .expect("Failed to read stdout");

            let out = String::from_utf8_lossy(&output.stdout);
            eprintln!("[PARENT] Raw AI output: {}", out);
            let resp: serde_json::Value = serde_json::from_str(&out).ok()?;

            let mv = resp.get("mv")?.as_array()?; // [[x1, y1], [x2, y2]]
            let from_arr = mv.get(0)?.as_array()?;
            let to_arr = mv.get(1)?.as_array()?;

            let from = (from_arr[0].as_u64()? as usize, from_arr[1].as_u64()? as usize);
            let to   = (to_arr[0].as_u64()? as usize, to_arr[1].as_u64()? as usize);

            Some((from, to))
        }
        _ => {
            // On timeout or error, kill and return None
            let _ = child.kill();
            None
        }
    }
}
