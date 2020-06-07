use raylib::prelude::*;

use crate::components::game::GameStates;
use crate::gui::gui_component::GuiComponentBehaviour;
use crate::gui::gui_button::Button;
use crate::gui::gui_cursor::Cursor;
use crate::consts::*;
use crate::gui::{gui_button::Button, gui_component::GuiComponentBehaviour, gui_cursor::Cursor};

pub struct Menu<'a> {
    pub current_state: &'a mut GameStates,
    start_game: bool,
}

impl Menu<'_> {
    pub fn new(initial_state: &mut GameStates) -> Menu {
        Menu {
            current_state: initial_state,
            start_game: false,
        }
    }

    pub fn change_state(&mut self, next_state: &str) {
        match next_state {
            "play" => {
                *self.current_state = GameStates::Playing;
            }
            "quitting" => {
                *self.current_state = GameStates::Quitting;
            }
            "menu" => {  // default (main menu)
                *self.current_state = GameStates::Menu;
            }
            _ => {}
        }
    }

    pub fn draw(&mut self, cursor: &Cursor, draw_handler: &mut RaylibDrawHandle,
                do_quit: &mut bool) {
        match self.start_game {
            true => self.change_state("play"),
            _ => {}
        }

        let mut start_button = Button::new(
            "Start".to_string(),
            Vector2::new(0_f32, (draw_handler.get_screen_height() as f32 / 2_f32) - 50_f32),
            Vector2::new(100_f32, 50_f32),
            Option::Some(&mut self.start_game),
            Color::GRAY,
            Color::DARKGRAY,
            Option::None,
        );

        let mut exit_button = Button::new(
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
