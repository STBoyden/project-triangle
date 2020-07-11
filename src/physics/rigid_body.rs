use super::physics_collider::PhysicsCollider;
use crate::types::Point;

pub trait RigidBody: PhysicsCollider {
    fn update_physics<T>(&mut self, others: &[T])
    where
        T: PhysicsCollider;

    fn move_pos(&mut self, deltas: Point);
    fn set_pos(&mut self, position: Point);
}
