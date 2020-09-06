use components::{entity::Entity, game::*, settings_loader::*};
use consts::{SCREEN_HEIGHT, SCREEN_WIDTH};

mod components;
mod consts;
mod physics;
mod types;

fn main() {
    if !check_settings() {
        create_settings();
    }

    let mut game_settings = GameSettings {
        resolution: (SCREEN_WIDTH, SCREEN_HEIGHT),
        is_fullscreen: false,
        show_fps: false,
    };

    read_settings(&mut game_settings).unwrap();

    let mut game = Game::new(
        "Project Triangle",
        game_settings.resolution.0,
        game_settings.resolution.1,
        GameStates::Menu,
    );

    let mut player = Entity::new(
        (
            game_settings.resolution.0 / 2,
            game_settings.resolution.1 / 2,
        ),
        (50, 50),
    );
    game.initialise(&mut player);
}
