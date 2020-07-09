use super::physics_body::PhysicsBody;
use super::physics_collider::PhysicsCollider;
use crate::types::Point;

pub trait RigidBody: PhysicsBody + PhysicsCollider {
    fn move_pos(&mut self, deltas: Point);
    fn set_pos(&mut self, position: Point);
}
