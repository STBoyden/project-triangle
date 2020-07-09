use raylib::prelude::*;

use crate::components::game::GameStates;
use crate::consts::*;
use crate::gui::{gui_button::Button, gui_component::GuiComponentBehaviour, gui_cursor::Cursor};

pub struct Menu<'a> {
    pub current_state: &'a mut GameStates,
}

impl Menu<'_> {
    pub fn new(initial_state: &mut GameStates) -> Menu {
        Menu {
            current_state: initial_state,
        }
    }

    pub fn draw(&mut self, cursor: &Cursor, draw_handler: &mut RaylibDrawHandle) {
        let mut buttons = vec![
            Button::new(
                "Start".to_string(),
                (0, (draw_handler.get_screen_height() / 2) - 50),
                (DEFAULT_BUTTON_WIDTH, DEFAULT_BUTTON_HEIGHT),
                "play".to_string(),
                Color::GRAY,
                Color::DARKGRAY,
                Option::None,
            ),
            Button::new(
                "Quit".to_string(),
                (0, (draw_handler.get_screen_height() / 2)),
                (DEFAULT_BUTTON_WIDTH, DEFAULT_BUTTON_HEIGHT),
                "quit".to_string(),
                Color::GRAY,
                Color::DARKGRAY,
                Option::None,
            ),
        ];

        for button in buttons.iter_mut() {
            button.draw(&cursor, draw_handler, &mut self.current_state);
        }
    }
}
