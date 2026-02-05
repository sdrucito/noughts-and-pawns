mod cli;

use std::io::{self, Write};
use cli::display::{print_game_state, print_instructions};
use cli::input::parse_command;
use game_core::game::game_state::GameState;
use game_core::game::rules::apply_move;

fn main() {
    let mut state = GameState::new();

    print_instructions();

    loop {
        print_game_state(&state);

        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mv = match parse_command(&input) {
            Ok(mv) => mv,
            Err(e) => {
                println!("Error: {}\n", e);
                continue;
            }
        };

        match apply_move(&mut state, mv) {
            Ok(_) => {}
            Err(e) => {
                println!("{}\n", e);
                if e.contains("wins the game") {
                    println!("Press ENTER to exit...");
                    let mut dummy = String::new();
                    let _ = io::stdin().read_line(&mut dummy);
                    break;
                }
            }
        }
    }
}
