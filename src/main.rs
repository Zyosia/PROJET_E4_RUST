use tournament_engine::Tournament;

fn main() {

    let mut tournament = Tournament::new();
    // Start the tournament
    tournament.start().unwrap();

    // Run the games
    tournament.run().unwrap();
}