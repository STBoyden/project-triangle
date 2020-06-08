pub mod game;
pub mod menu;
pub mod pause_menu;
pub mod entity;

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
