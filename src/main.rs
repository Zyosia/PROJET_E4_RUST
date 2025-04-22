use gui::run_graphics;
use terminal_gui::terminal_helper_functions::*;
use terminal_gui::run_terminal_ui;

fn main(){
    loop {
        match menu() {
            1 => run_terminal_ui(),
            2 => run_graphics(),
            3 => {
                println!("👋 Fermeture... 🚪");
                break;
            }
            _ => {
                println!("❌ Choix invalide. Veuillez réessayer");
                pause();
            }
        }
    }
    
}

/// Displays the menu to the user and prompts for a choice.
/// Returns the user's choice as a u32 value.
fn menu() -> u32 {
    clear_screen();
    println!("🎮 Choississez une méthode de lancement : 🎮");
    println!("1️⃣  Interface Terminal");
    println!("2️⃣  Interface Graphique");
    println!("3️⃣  Quitter");
    print!("👉 Saisissez votre choix: ");
    read_input()
}