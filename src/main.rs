use raylib::core::math::Vector2;

mod components;
mod consts;
mod gui;

use components::{entity::Entity, game::*};
use consts::{SCREEN_HEIGHT, SCREEN_WIDTH};
use gui::gui_cursor::Cursor;

fn main() {
    let mut player = Entity::new(Vector2::new(
        (SCREEN_WIDTH / 2) as f32,
        (SCREEN_HEIGHT / 2) as f32,
    ));

    let mut cursor = Cursor::new(Vector2::new(
        (SCREEN_WIDTH / 2) as f32,
        (SCREEN_HEIGHT / 2) as f32,
    ));

    let mut title = String::from("Triangular Tribulations");
    let mut initial_state = GameStates::Menu;
    let mut game = Game::new(
        &mut player,
        &mut cursor,
        &mut title,
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        &mut initial_state,
    );

    game.initialise();
}
