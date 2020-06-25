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

    pub fn move_pos(&mut self, deltas: (f32, f32)) {
        self.position.x += deltas.0;
        self.position.y += deltas.1;
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
