use crate::physics::{physics_body::PhysicsBody, physics_collider::PhysicsCollider};
use ptgui::prelude::*;
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
}

impl Drawable for Entity {
    fn draw(&mut self, draw_handler: &mut RaylibDrawHandle) {
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

impl PhysicsBody for Entity {
    fn try_fall<T: PhysicsCollider>(&mut self, others: &[T]) {
        let fall_speed = 10; //0;

        self.try_move((0, fall_speed), others);
    }

    fn try_move<T: PhysicsCollider>(&mut self, deltas: Point, others: &[T]) {
        self.move_pos(deltas);
        let res = others.iter().map(|other| other.is_colliding(*self));

        let mut can_move = true;

        res.for_each(|result| {
            if result {
                can_move = false;
            }
        });

        if !can_move {
            self.move_pos((-deltas.0, -deltas.1));
        }
    }

    fn move_pos(&mut self, deltas: Point) {
        self.position.0 += deltas.0;
        self.position.1 += deltas.1;
    }

    fn set_pos(&mut self, position: Point) {
        self.position = position;
    }
}
