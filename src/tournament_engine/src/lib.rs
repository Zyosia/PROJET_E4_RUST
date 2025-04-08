use std::{sync::Arc, collections::HashMap, fs, path::Path, io};
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
    tournament_name: String,
    state: TournamentState,
    participants: Vec<String>,  // Names of AI participants
    ai_functions: HashMap<String, (Arc<Library>, Arc<Box<dyn Fn(&Board, char) -> Option<((usize, usize), (usize, usize))>>>)>,
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
            ai_functions: HashMap::new(),
            scores: HashMap::new(),
            nombre_parties: 1,
        }
    }

    /// Returns the current state of the tournament.
    pub fn get_state(&self) -> TournamentState {
        self.state.clone()
    }

    /// Returns the tournament name
    pub fn get_name(&self) -> String {
        self.tournament_name.clone()
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
    fn add_participant(
        &mut self, 
        ai_name: String, 
        lib: Arc<Library>,
        ai_function: Box<dyn Fn(&Board, char) -> Option<((usize, usize), (usize, usize))>>
    ){
        self.ai_functions.insert(ai_name.clone(), (lib, Arc::new(ai_function)));
        self.participants.push(ai_name);
    }

    /// Loads all AI participants into the tournament.
    /// # Returns
    /// - `io::Result<(), String>`: Returns `Ok(())` if successful, otherwise returns an error.
    pub fn add_all_ai(&mut self) -> Result<(), String>{
        // Charge les IA
        let mut ai_functions_data = load_ai_functions().unwrap();
        if ai_functions_data.is_empty(){
            return Err("Erreur: Aucune IA chargée.".to_string());
        }

        loop{
            if ai_functions_data.is_empty(){
                break;
            }
            let ai_data = ai_functions_data.pop();
            let Some((lib, ai_function, ai_name)) = ai_data.unwrap() else {panic!("Erreur: impossible de récupéré les données de l'IA")};
            
            if self.ai_functions.contains_key(&ai_name) == false {
                Self::add_participant(self, ai_name, lib, ai_function);
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

        // Compilation des IA
        if let Err(e) = compile_all_ai() {
            eprintln!("❌ Erreur lors de la compilation des IA: {}", e);
            return Err("Fatal Error".to_string());
        }
        println!("✅ Toutes les IA ont été compilées avec succès !");

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
                        for _i in 0..self.nombre_parties {
                            game.run().unwrap();
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
}