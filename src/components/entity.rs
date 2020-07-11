use crate::physics::{physics_collider::PhysicsCollider, rigid_body::RigidBody};
use crate::types::*;
use raylib::prelude::*;
use serde::Deserialize;

#[derive(PartialEq, Copy, Clone, Deserialize, Debug)]
pub struct Entity {
    position: Point,
    size: Dimensions,
}

impl Entity {
    pub fn new(position: Point, size: Dimensions) -> Self {
        Entity { position, size }
    }

    pub fn draw(&mut self, draw_handler: &mut RaylibDrawHandle) {
        draw_handler.draw_rectangle(
            self.position.0,
            self.position.1,
            self.size.0,
            self.size.1,
            Color::RED,
        );
    }
}

impl PhysicsCollider for Entity {
    fn get_pos(&self) -> Point {
        self.position
    }

    fn get_size(&self) -> Dimensions {
        self.size
    }
}

impl RigidBody for Entity {
    fn update_physics<T: PhysicsCollider>(&mut self, others: &[T]) {
        others.into_iter().for_each(|other| {
            if !other.is_colliding(*self) {
                self.move_pos((0, 9));
            }
        });
    }

    fn move_pos(&mut self, deltas: Point) {
        self.position.0 += deltas.0;
        self.position.1 += deltas.1;
    }

    fn set_pos(&mut self, position: Point) {
        self.position = position;
    }
}
