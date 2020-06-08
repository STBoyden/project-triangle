use raylib::core::math::Vector2;

mod components;
mod gui;
mod consts;

use components::{game::*, entity::Entity};
use gui::gui_cursor::Cursor;

fn main() {
    let screen_width = 1280;
    let screen_height = 720;

    let mut player = Player::new(Vector2::new(
    let mut player = Entity::new(Vector2::new(
        (screen_width / 2) as f32,
        (screen_height / 2) as f32,
    ));
    let mut cursor = Cursor::new(Vector2::new(
        (screen_width / 2) as f32,
        (screen_height / 2) as f32,
    ));

    let mut title = String::from("Triangular Tribulations");
    let mut initial_state = GameStates::Menu;
    let mut game = Game::new(&mut player, &mut cursor, &mut title, &mut initial_state);

    game.initialise(screen_width, screen_height);
}
