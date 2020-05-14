use raylib::prelude::*;

pub struct Player {
    pub position: Vector2,
    pub size: i32,
}

impl Player {
    pub fn new(initial_position: Vector2) -> Self {
        Player {
            position: initial_position,
            size: 20,
        }
    }

    pub fn _draw(&mut self, draw_handler: &mut RaylibDrawHandle) {
        draw_handler.draw_rectangle(self.position.x as i32, self.position.y as i32, self.size,
                                    self.size, Color::RED);
    }
}
