use std::{collections::HashMap, fs, path::Path, io};
use game_engine::game::Game;
use ai_manager::{ai_loader::{compile_all_ai, find_all_ai_plugin_paths}, ai_creator::create_new_ai};
use bevy::prelude::Resource;

#[derive(Clone, Debug, PartialEq)]
pub enum TournamentState {
    Waiting,
    Started,
}

#[derive(Resource)]
pub struct Tournament {
    tournament_name: String,
    state: TournamentState,
    participants: Vec<String>,  // Names of AI participants
    ai_lib_paths: HashMap<String, String>,
    scores: HashMap<String, usize>,
    nombre_parties: i32,
}

impl Tournament {

    /// Creates a new tournament instance.
    pub fn new() -> Self {
        Self {
            tournament_name: String::new(),
            state: TournamentState::Waiting,
            participants: Vec::new(),
            ai_lib_paths: HashMap::new(),
            scores: HashMap::new(),
            nombre_parties: 10,
        }
    }

    /// Returns the current state of the tournament.
    pub fn get_state(&self) -> TournamentState {
        self.state.clone()
    }

    pub fn get_state_to_string(&self) -> String {
        if self.state == TournamentState::Waiting {
            return "En Attente".to_string().clone();
        }else{
            return "Démarré".to_string().clone();
        }
    }

    /// Returns the tournament name
    pub fn get_name(&self) -> String {
        self.tournament_name.clone()
    }

    /// Returns the list of participants in the tournament.
    pub fn get_participants(&self) -> Vec<String> {
        self.participants.clone()
    }

    /// Returns the total number of games that will be played in the tournament.
    pub fn get_total_number_of_games(&self) -> usize {
        let n = self.participants.len();
        let k = self.nombre_parties;
        n * (n - 1) / 2 * k as usize
    }

    /// Returns the score for a specific AI participant.
    /// # Arguments
    /// - `name`: The name of the AI.
    /// # Returns
    /// - `Some(score)` if the AI exists, otherwise `None`.
    pub fn get_score_for_ai(&self, name: &str) -> Option<usize> {
        self.scores.get(name).copied()
    }

    /// Returns all the scores for every AI participant.
    /// # Returns
    /// - A clone of the internal scores HashMap.
    pub fn get_all_scores(&self) -> HashMap<String, usize> {
        self.scores.clone()
    }

    /// Set the tournament name
    /// # Arguments
    /// - `name` - The name of the tournament.
    pub fn set_tournament_name(&mut self, name: String) -> Result<(), String> {
        if self.state == TournamentState::Started{
            return Err("❌ Impossible de changer le nombre de partie le tournois a déjà commencé !".to_string());
        }
        self.tournament_name = name;
        Ok(())
    }

    /// Set the number of game played
    /// # Arguments
    /// - `nombre_parties` - The number of game played.
    pub fn set_number_of_games(&mut self, nombre_parties: i32) -> Result<(), String>{
        self.nombre_parties = nombre_parties;
        Ok(())
    }

    pub fn get_number_of_games(&self) -> i32{
        self.nombre_parties.clone()
    }

    /// Adds a new AI to the tournament.
    /// Returns an error if the tournament has already started or the ai creation has failed.
    /// # Returns
    /// - `io::Result<(), String>`: Returns `Ok(())` if successful, otherwise returns an error.
    pub fn add_ia(&mut self) -> Result<(), String>{
        if self.state == TournamentState::Started {
            return Err("❌ Impossible d'ajouter une IA le tournois a déjà commencé !".to_string());
        }
        if let Err(e) = create_new_ai(){
            eprintln!("❌ Erreur lors de la création de l'IA: {}", e);
            return Err("Fatal Error".to_string());
        }
        println!("✅ IA ajoutée avec succès !");

        Self::add_all_ai(self).unwrap();
        
        Ok(())
    }

    /// Adds a participant to the tournament.
    pub fn add_participant(
        &mut self, 
        ai_name: String, 
        ai_lib_path: String,
    ){
        self.ai_lib_paths.insert(ai_name.clone(), ai_lib_path.clone());
        self.participants.push(ai_name);
    }

    /// Loads all AI participants into the tournament.
    /// # Returns
    /// - `io::Result<(), String>`: Returns `Ok(())` if successful, otherwise returns an error.
    pub fn add_all_ai(&mut self) -> Result<(), String>{

        // Compilation des IA
        if let Err(e) = compile_all_ai() {
            eprintln!("❌ Erreur lors de la compilation des IA: {}", e);
            return Err("Fatal Error".to_string());
        }
        
        println!("✅ Toutes les IA ont été compilées avec succès !");

        // Charge les IA

        let mut paths = find_all_ai_plugin_paths();
        if paths.is_empty(){
            return Err("Erreur: Aucune IA chargée.".to_string());
        }
        println!("{:?}", paths);

        loop{
            if paths.is_empty(){
                break;
            }
            let ai_data = paths.pop();
            let (ai_name, ai_lib_path) = ai_data.unwrap();
            
            if self.ai_lib_paths.contains_key(&ai_name) == false {
                Self::add_participant(self, ai_name, ai_lib_path);
            }

        }
        Ok(())
    }

    /// Starts the tournament.
    /// Loads and Compiles AI functions and initializes the tournament state.
    /// # Returns
    /// - `io::Result<(), String>`: Returns `Ok(())` if successful, otherwise returns an error.
    pub fn start(&mut self) -> Result<(), String> {

        if self.state == TournamentState::Started {
            return Err("❌ Le tournoi a déjà commencé.".to_string());
        }

        // Ajout des participants
        Self::add_all_ai(self).unwrap();

        if self.participants.len() < 2 {
            return Err("❌ Pas assez de participants.".to_string());
        }

        // Set the state to Started
        self.state = TournamentState::Started;

        // Initialize scores for all participants
        for participant in &self.participants {
            self.scores.insert(participant.clone(), 0);
        }

        Self::show_participants(&self);

        Ok(())
    }

    /// Runs the tournament by making each AI play against the others.
    /// # Returns
    /// - `io::Result<(), String>`: Returns `Ok(())` if successful, otherwise returns an error.
    pub fn run(&mut self) -> Result<(), String> {
        if self.state != TournamentState::Started {
            return Err("Le tournois n'a pas commencé.".to_string());
        }

        // Run the games: each participant plays against each other
        for i in 0..self.participants.len() {
            for j in (i + 1)..self.participants.len() {
                let ai_name1 = &self.participants[i];
                let ai_name2 = &self.participants[j];

                let ai1_lib_path = self.ai_lib_paths.get(ai_name1);
                let ai2_lib_path = self.ai_lib_paths.get(ai_name2);

                let mut game = Game::new(
                    ai_name1.clone(),
                    ai_name2.clone(),
                    ai1_lib_path.unwrap().clone(),
                    ai2_lib_path.unwrap().clone(),
                );
                for i in 0..self.nombre_parties {
                    game.run(i + 1).unwrap();
                    let winner = game.get_winner();
                    if winner == 'X' {
                        *self.scores.get_mut(ai_name1).unwrap() += 1;
                    } else if winner == 'O' {
                        *self.scores.get_mut(ai_name2).unwrap() += 1;
                    }
                }
            }
        }

        // Display the results (scoreboard)
        self.display_scoreboard();

        Ok(())
    }

    /// Displays the tournament scoreboard with rankings.
    pub fn display_scoreboard(&self) {
        let mut sorted_scores: Vec<(&String, &usize)> = self.scores.iter().collect();
        sorted_scores.sort_by(|a, b| b.1.cmp(a.1)); // Sort descending

        println!("\n🏆 Résultats du Tournoi 🏆");
        for (rank, (ai_name, score)) in sorted_scores.iter().enumerate() {
            println!("{}. {} - {} points", rank + 1, ai_name, score);
        }
    }

    /// Displays the list of AI participants in the tournament.
    pub fn show_participants(&self) {
        if self.participants.is_empty() {
            println!("❌ Aucun participant enregistré.");
            return;
        }
        println!("\n🤖 Participants du tournoi:");
        for (i, ai) in self.participants.iter().enumerate() {
            println!("{}. {}", i + 1, ai);
        }
    }
    /// Save all `.txt` files from `games_history` to `tournament_save`
    pub fn save_tournament(&self) -> io::Result<()> {
        let source_folder = "src/games_history";
        let destination_folder = format!("src/.tournament_saves/{}", self.tournament_name);

        // Ensure the destination folder exists
        if !Path::new(&destination_folder).exists() {
            fs::create_dir(&destination_folder)?;
        }

        // Read all entries in the source folder
        for entry in fs::read_dir(source_folder)? {
            let entry = entry?;
            let path = entry.path();

            // Check if the entry is a file and ends with .txt
            if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("txt") {
                let file_name = path.file_name().unwrap(); // Get the file name
                let new_path = Path::new(&destination_folder).join(file_name); // Destination path

                // Move the file
                fs::rename(&path, &new_path)?;

                println!("✅ {} saved.", file_name.to_string_lossy());
            }
        }

        println!("✅ {} saved", self.tournament_name);
        Ok(())
    }

    /// Resets the tournament to its initial state.
    pub fn reset(&mut self) {
        self.state = TournamentState::Waiting;
        self.tournament_name.clear();
        self.participants.clear();
        self.ai_lib_paths.clear();
        self.scores.clear();
        self.nombre_parties = 10;
        println!("🔄 Tournoi réinitialisé !");
    }
    
}