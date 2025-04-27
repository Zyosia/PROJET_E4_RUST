use std::fs::File;
use std::io::{self, BufRead};
use crate::tournament::Tournament;
use std::collections::HashMap;

pub fn load_tournament_from_file(filename: &str) -> io::Result<Tournament> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut tournament = Tournament::new();
    let mut participants = Vec::new();
    let mut ai_lib_paths = HashMap::new();
    let mut scores = HashMap::new();

    let mut line_iter = reader.lines();

    // Read the tournament name
    if let Some(Ok(line)) = line_iter.next() {
        tournament.set_tournament_name(line.split(": ").nth(1).unwrap().to_string()).unwrap();
    }

    // Read the tournament state
    if let Some(Ok(line)) = line_iter.next() {
        let state = line.split(": ").nth(1).unwrap();
        tournament.set_state_from_string(state.to_string());
    }

    // Read participants
    if let Some(Ok(line)) = line_iter.next() {
        if line == "Participants:" {
            while let Some(Ok(line)) = line_iter.next() {
                if line.trim().is_empty() {
                    break;
                }
                participants.push(line.trim().to_string());
            }
        }
    }

    // Read AI library paths
    if let Some(Ok(line)) = line_iter.next() {
        if line == "AI Library Paths:" {
            while let Some(Ok(line)) = line_iter.next() {
                if line.trim().is_empty() {
                    break;
                }
                let parts: Vec<&str> = line.split(" -> ").collect();
                if parts.len() == 2 {
                    ai_lib_paths.insert(parts[0].to_string(), parts[1].to_string());
                }
            }
        }
    }

    // Read scores
    if let Some(Ok(line)) = line_iter.next() {
        if line == "Scores:" {
            while let Some(Ok(line)) = line_iter.next() {
                if line.trim().is_empty() {
                    break;
                }
                let parts: Vec<&str> = line.split(" -> ").collect();
                if parts.len() == 2 {
                    let score = parts[1].parse::<usize>().unwrap_or(0);
                    scores.insert(parts[0].to_string(), score);
                }
            }
        }
    }

    // Read the number of games
    if let Some(Ok(line)) = line_iter.next() {
        let parts: Vec<&str> = line.split(": ").collect();
        if parts.len() == 2 {
            tournament.set_number_of_games(parts[1].parse().unwrap()).unwrap();
        }
    }

    // Set the participants and AI library paths
    tournament.set_participants(participants);
    tournament.set_ai_lib_paths(ai_lib_paths);
    tournament.set_scores(scores);

    Ok(tournament)
}
