pub mod entity;
pub mod game;
pub mod map;
pub mod map_gen;
pub mod menu;
pub mod object;
pub mod pause_menu;

use game::GameStates;

pub fn change_state(state: &mut GameStates, state_str: &str) {
    match state_str {
        "menu" => *state = GameStates::Menu,
        "paused" => *state = GameStates::Paused,
        "quit" => *state = GameStates::Quitting,
        "play" => *state = GameStates::Playing,
        _ => {}
    }
}
