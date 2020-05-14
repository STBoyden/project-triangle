use raylib::prelude::*;

pub struct Cursor {
    pub position: Vector2,
    pub is_clicked: bool,
}

impl Cursor {
    pub fn new(initial_position: Vector2) -> Self {
        Cursor {
            position: initial_position,
            is_clicked: false,
        }
    }

    pub fn draw(&mut self, draw_handler: &mut RaylibDrawHandle, sprite: &Texture2D) {
        self.position = draw_handler.get_mouse_position();
        self.is_clicked = draw_handler.is_mouse_button_released(
            consts::ffi::MouseButton::MOUSE_LEFT_BUTTON);
        draw_handler.draw_texture(sprite, self.position.x as i32, self.position.y as i32,
                                  Color::WHITE);
    }
}
