use crate::cli::display::{print_game_state, print_instructions};
use crate::game::game_state::GameState;


mod cli;
mod game;



fn main() {
    let state = GameState::new();

    print_instructions();
    print_game_state(&state);
}
