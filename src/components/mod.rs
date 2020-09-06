pub mod entity;
pub mod game;
pub mod map;
pub mod map_gen;
pub mod object;
pub mod settings_loader;

extern crate regex;
use crate::{load_settings, update_settings};
use game::GameStates;

pub fn change_state(state: &mut GameStates, state_str: &str) {
    match state_str {
        "menu" => *state = GameStates::Menu,
        "paused" => *state = GameStates::Paused,
        "quit" => *state = GameStates::Quitting,
        "play" => *state = GameStates::Playing,
        "play_reset" => *state = GameStates::Resetting,
        "settings_menu" => *state = GameStates::Settings,
        "settings_menu_toggle_fps" => {
            let mut settings = load_settings().unwrap();
            settings.show_fps = !settings.show_fps;
            update_settings(&settings).unwrap();
            *state = GameStates::Settings;
        }
        "settings_menu_toggle_fullscreen" => {
            let mut settings = load_settings().unwrap();
            settings.is_fullscreen = !settings.is_fullscreen;
            update_settings(&settings).unwrap();
            *state = GameStates::Settings;
        }
        _ => {}
    }
}
