use std::{sync::Arc, collections::HashMap};
use libloading::{Library};
use board::Board;
use game_engine::Game;
use ai_manager::{load_ai_functions, compile_all_ai, create_new_ai};

#[derive(Clone, Debug, PartialEq)]
pub enum TournamentState {
    Waiting,
    Started,
}

pub struct Tournament {
    state: TournamentState,
    participants: Vec<String>,  // Names of AI participants
    ai_functions: HashMap<String, (Arc<Library>, Arc<Box<dyn Fn(&Board, char) -> Option<((usize, usize), (usize, usize))>>>)>,
    scores: HashMap<String, usize>,
}

impl Tournament {
    pub fn new() -> Self {
        Self {
            state: TournamentState::Waiting,
            participants: Vec::new(),
            ai_functions: HashMap::new(),
            scores: HashMap::new(),
        }
    }

    pub fn get_state(&self) -> TournamentState {
        self.state.clone()
    }


    pub fn add_ia(&mut self) -> Result<(), String>{
        if self.state == TournamentState::Started {
            return Err("❌ Impossible d'ajouter une IA le tournois a déjà commencé !".to_string());
        }
        if let Err(e) = create_new_ai(){
            eprintln!("❌ Erreur lors de la création de l'IA: {}", e);
            return Err("Fatal Error".to_string());
        }
        println!("✅ IA ajoutée avec succès !");
        Ok(())
    }

    fn add_participant(
        &mut self, 
        ai_name: String, 
        lib: Arc<Library>,
        ai_function: Box<dyn Fn(&Board, char) -> Option<((usize, usize), (usize, usize))>>
    ){
        self.ai_functions.insert(ai_name.clone(), (lib, Arc::new(ai_function)));
        self.participants.push(ai_name);
    }

    fn add_all_ai(&mut self) -> Result<(), String>{
        // Charge les IA
        let mut ai_functions_data = load_ai_functions().unwrap();
        if ai_functions_data.is_empty(){
            return Err("No AI functions loaded.".to_string());
        }

        loop{
            if ai_functions_data.is_empty(){
                break;
            }
            let ai_data = ai_functions_data.pop();
            let Some((lib, ai_function, ai_name)) = ai_data.unwrap() else {panic!("ai_data is None")};
            Self::add_participant(self, ai_name, lib, ai_function);
        }
        Ok(())
    }

    pub fn start(&mut self) -> Result<(), String> {

        if self.state == TournamentState::Started {
            return Err("❌ Le tournois a déjà commencé.".to_string());
        }

        // Compilation des IA
        if let Err(e) = compile_all_ai() {
            eprintln!("❌ Erreur lors de la compilation des IA: {}", e);
            return Err("Fatal Error".to_string());
        }
        println!("✅ Toutes les IA ont été compilées avec succès !");

        // Ajout des participants
        Self::add_all_ai(self).unwrap();

        if self.participants.len() < 2 {
            return Err("❌ Not enough participants to start the tournament.".to_string());
        }

        // Set the state to Started
        self.state = TournamentState::Started;

        // Initialize scores for all participants
        for participant in &self.participants {
            self.scores.insert(participant.clone(), 0);
        }

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), String> {
        if self.state != TournamentState::Started {
            return Err("Le tournois n'a pas commencé.".to_string());
        }

        // Run the games: each participant plays against each other
        for i in 0..self.participants.len() {
            for j in (i + 1)..self.participants.len() {
                let ai_name1 = &self.participants[i];
                let ai_name2 = &self.participants[j];

                if let Some((lib1, ai_function1)) = self.ai_functions.get(ai_name1) {
                    if let Some((lib2, ai_function2)) = self.ai_functions.get(ai_name2) {
                        let ai_function1 = Arc::clone(ai_function1);
                        let ai_function2 = Arc::clone(ai_function2);

                        let mut game = Game::new(
                            ai_name1.clone(),
                            ai_name2.clone(),
                            Some(ai_function1),  // Pass the cloned Arc
                            Some(ai_function2),  // Pass the cloned Arc
                            Some(lib1.clone()),
                            Some(lib2.clone()),
                        );

                        game.run().unwrap();

                        // Assuming we have a method to determine the winner
                        let winner = game.get_board().check_winner().unwrap_or(' ');

                        if winner == 'X' {
                            *self.scores.get_mut(ai_name1).unwrap() += 1;
                        } else if winner == 'O' {
                            *self.scores.get_mut(ai_name2).unwrap() += 1;
                        }
                    }
                }
            }
        }

        // Display the results (scoreboard)
        self.display_scoreboard();

        Ok(())
    }

    pub fn display_scoreboard(&self) {
        let mut sorted_scores: Vec<(&String, &usize)> = self.scores.iter().collect();
        sorted_scores.sort_by(|a, b| b.1.cmp(a.1)); // Sort descending

        println!("\n🏆 Tournament Results 🏆");
        for (rank, (ai_name, score)) in sorted_scores.iter().enumerate() {
            println!("{}. {} - {} points", rank + 1, ai_name, score);
        }
    }

    pub fn show_participants(&self) {
        if self.participants.is_empty() {
            println!("❌ No participants registered.");
            return;
        }
        println!("\n🤖 Tournament Participants:");
        for (i, ai) in self.participants.iter().enumerate() {
            println!("{}. {}", i + 1, ai);
        }
    }
}
