use ptgui::prelude::*;
use raylib::prelude::*;
use serde::Deserialize;

use crate::physics::physics_collider::PhysicsCollider;

#[derive(Deserialize, Copy, Clone, Debug)]
pub struct Object {
    position: Point,
    size: Dimensions,
}

impl Drawable for Object {
    fn draw(&mut self, draw_handler: &mut RaylibDrawHandle) {
        draw_handler.draw_rectangle(
            self.position.0,
            self.position.1,
            self.size.0,
            self.size.1,
            Color::DARKBLUE,
        );
    }
}

impl PhysicsCollider for Object {
    fn get_pos(&self) -> Point {
        self.position
    }

    fn get_size(&self) -> Dimensions {
        self.size
    }
}
