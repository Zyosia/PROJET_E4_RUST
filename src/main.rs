use std::process::exit;

use ai_manager::load_ai_functions;
use game_engine::Game;

fn main() {
    // Load the AI functions
    let mut ai_functions_data = load_ai_functions().unwrap();

    if ai_functions_data.is_empty(){
        eprintln!("No AI functions loaded.");
        exit(1);
    }
    let ai1_data = ai_functions_data.pop(); // Récupère et retire la dernière IA
    let ai2_data = ai_functions_data.pop(); // Récupère et retire l'avant-dernière IA

    let Some((lib1, ai1)) = ai1_data.unwrap() else {panic!("ai1_data none")};
    let Some((lib2, ai2)) = ai2_data.unwrap() else {panic!("ai2_data none")};

    let mut game = Game::new(Some(lib1),Some(ai1), Some(lib2), Some(ai2));
    game.run();
    
}


/*
fn main() {
    let ai_data = load_ai_functions();

    // Ensure ai_data is Some and unwrap it safely
    let mut ai_data = match ai_data {
        Some(data) => data,
        None => {
            eprintln!("Erreur : Impossible de charger les IA.");
            exit(1);
        }
    };

    if ai_data.is_empty() {
        eprintln!("Aucune IA trouvée dans le dossier 'ai/'. Ajoutez un fichier .so.");
        exit(1);
    }

    if ai_data.len() < 2 {
        eprintln!("Pas assez d'IA chargées. Ajoutez plus d'IA.");
        exit(1);
    }

    let ai_function = ai_data.remove(0);
    let ai_function2 = ai_data.remove(0);

    println!("L'IA a été chargée avec succès.");

    let mut game = Game::new(
        Some(ai_function), // IA 1
        Some(ai_function2) // IA 2
    );

    game.run();
}
*/