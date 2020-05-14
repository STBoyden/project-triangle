use raylib::prelude::*;

use crate::gui::gui_component::GuiComponentBehaviour;

pub struct Menu {
    pub current_state: super::game::GameStates,
}

impl Menu {
    pub fn new() -> Self {
        Menu {
            current_state: super::game::GameStates::Menu,
        }
    }

    pub fn change_state(&mut self, next_state: &str) {
        match next_state {
            "play" => {
                self.current_state = super::game::GameStates::Playing;
            }
            "pause" => {
                self.current_state = super::game::GameStates::Paused;
            }
            "quitting" => {
                self.current_state = super::game::GameStates::Quitting;
            }
            "menu" => {  // default (main menu)
                self.current_state = super::game::GameStates::Menu;
            }
            _ => {}
        }
    }

    pub fn draw(&mut self, cursor: &super::gui::gui_cursor::Cursor,
                draw_handler: &mut RaylibDrawHandle, do_quit: &mut bool) {
        let mut start_button = super::gui::gui_button::Button::new(
            "Start".to_string(),
            Vector2::new(0_f32, (draw_handler.get_screen_height() as f32 / 2_f32) - 50_f32),
            Vector2::new(100_f32, 50_f32),
            Option::None,
            Color::GRAY,
            Color::DARKGRAY,
            Option::None,
        );

        let mut exit_button = super::gui::gui_button::Button::new(
            "Quit".to_string(),
            Vector2::new(0_f32, start_button.position.y + 50_f32),
            Vector2::new(100_f32, 50_f32),
            Option::Some(do_quit),
            Color::GRAY,
            Color::DARKGRAY,
            Option::None,
        );

        start_button.draw(cursor, draw_handler);
        exit_button.draw(cursor, draw_handler);
    }
}
