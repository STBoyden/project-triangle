use raylib::prelude::*;

use super::game::GameStates;
use crate::consts::*;
use crate::gui::{gui_button::Button, gui_component::GuiComponentBehaviour, gui_cursor::Cursor};

pub struct PauseMenu<'a> {
    pub current_state: &'a mut GameStates,
}

impl PauseMenu<'_> {
    pub fn new(initial_state: &mut GameStates) -> PauseMenu {
        PauseMenu {
            current_state: initial_state,
        }
    }

    pub fn draw(&mut self, cursor: &Cursor, draw_handler: &mut RaylibDrawHandle) {
        let mut buttons = vec![
            Button::new(
                "Resume".to_string(),
                Vector2::new(
                    0_f32,
                    (draw_handler.get_screen_height() as f32 / 2_f32) - 50_f32,
                ),
                Vector2::new(150_f32, DEFAULT_BUTTON_HEIGHT),
                "play".to_string(),
                Color::GRAY,
                Color::DARKGRAY,
                Option::None,
            ),
            Button::new(
                "Quit to menu".to_string(),
                Vector2::new(0_f32, draw_handler.get_screen_height() as f32 / 2_f32),
                Vector2::new(150_f32, DEFAULT_BUTTON_HEIGHT),
                "menu".to_string(),
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
