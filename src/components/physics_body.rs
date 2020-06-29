pub trait PhysicsBody {
    fn update_physics(&mut self);
    fn is_collding<T>(&self, other: T) -> bool
    where
        T: PhysicsBody;
}
