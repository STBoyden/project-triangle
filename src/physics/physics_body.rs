use super::physics_collider::PhysicsCollider;
use ptgui::prelude::*;

pub trait PhysicsBody: PhysicsCollider {
    fn try_fall<T>(&mut self, others: &[T])
    where
        T: PhysicsCollider;

    fn try_move<T>(&mut self, deltas: Point, others: &[T])
    where
        T: PhysicsCollider;
    fn move_pos(&mut self, deltas: Point);
    fn set_pos(&mut self, position: Point);
}
