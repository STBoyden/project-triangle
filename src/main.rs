mod components;
mod consts;
mod gui;
mod physics;
mod types;

use components::{entity::Entity, game::*};
use consts::{SCREEN_HEIGHT, SCREEN_WIDTH};
use gui::gui_cursor::Cursor;

fn main() {
    let mut player = Entity::new((SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2), (50, 50));

    let mut cursor = Cursor::new((SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2));

    let mut title = String::from("Project Triangle");
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
