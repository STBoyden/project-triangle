use raylib::prelude::*;

#[derive(PartialEq, Copy, Clone)]
pub struct Entity {
    pub position: Vector2,
    pub size: i32,
}

impl Entity {
    pub fn new(initial_position: Vector2) -> Self {
        Entity {
            position: initial_position,
            size: 20,
        }
    }

    pub fn move_pos(&mut self, handler: &RaylibHandle, speed: f32, is_vertical: bool) {
        if is_vertical {
            self.position.y += speed * (handler.get_frame_time() * 100.0) as f32;
        } else {
            self.position.x += speed * (handler.get_frame_time() * 100.0) as f32;
        }
    }

    pub fn draw(&mut self, draw_handler: &mut RaylibDrawHandle) {
        draw_handler.draw_rectangle(
            self.position.x as i32,
            self.position.y as i32,
            self.size,
            self.size,
            Color::RED,
        );
    }
}
