use consts::ffi::MouseButton::*;
use raylib::prelude::*;

use crate::types::Point;

pub struct Cursor {
    pub position: Point,
    pub is_clicked: bool,
}

impl Cursor {
    pub fn new(initial_position: Point) -> Self {
        Cursor {
            position: initial_position,
            is_clicked: false,
        }
    }

    pub fn draw(&mut self, draw_handler: &mut RaylibDrawHandle, sprite: &Texture2D, do_draw: bool) {
        self.position = (
            draw_handler.get_mouse_position().x as i32,
            draw_handler.get_mouse_position().y as i32,
        );
        self.is_clicked = draw_handler.is_mouse_button_released(MOUSE_LEFT_BUTTON);

        if do_draw {
            draw_handler.draw_texture(sprite, self.position.0, self.position.1, Color::WHITE);
        }
    }
}
