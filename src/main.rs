use raylib::core::math::Vector2;

mod game;
mod player;
mod menu;
mod gui;

fn main() {
    let screen_width = 1280;
    let screen_height = 720;

    let mut player = player::Player::new(
        Vector2::new((screen_width / 2) as f32, (screen_height / 2) as f32)
    );
    let mut cursor = gui::gui_cursor::Cursor::new(
        Vector2::new((screen_width / 2) as f32, (screen_height / 2) as f32)
    );
    let mut title = String::from("Triangular Tribulations");
    let menu = menu::Menu::new();
    let mut game = game::Game {
        player: &mut player,
        cursor: &mut cursor,
        title: &mut title,
        menu,
    };

    game.initialise(screen_width, screen_height);
}
