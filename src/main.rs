use std::process::exit;
use ai_manager::{load_ai_functions, compile_all_ai, create_new_ai};
use game_engine::Game;

fn main() {

    /*
    //Ajouter une IA test
    if let Err(e) = create_new_ai(){
        eprintln!("❌ Erreur lors de la création de l'IA: {}", e);
        exit(1); // Arrêter immédiatement le programme
    }
    println!("✅ IA ajoutée avec succès !");
    */
    
    // Compilation des IA
    if let Err(e) = compile_all_ai() {
        eprintln!("❌ Erreur lors de la compilation des IA: {}", e);
        exit(1); // Arrêter immédiatement le programme
    }
    println!("✅ Toutes les IA ont été compilées avec succès !");

    // Charge les IA
    let mut ai_functions_data = load_ai_functions().unwrap();

    if ai_functions_data.is_empty(){
        eprintln!("No AI functions loaded.");
        exit(1);
    }
    let ai1_data = ai_functions_data.pop(); // Récupère et retire la dernière IA
    let ai2_data = ai_functions_data.pop(); // Récupère et retire l'avant-dernière IA

    let Some((lib1, ai1)) = ai1_data.unwrap() else {panic!("ai1_data none")};
    let Some((lib2, ai2)) = ai2_data.unwrap() else {panic!("ai2_data none")};

    let mut game = Game::new(Some(ai1), Some(ai2), Some(lib1), Some(lib2));
    game.run();
}