use crate::types::*;

pub trait PhysicsCollider {
    fn is_colliding<T>(&self, other: T) -> bool
    where
        T: PhysicsCollider,
    {
        let t_size = self.get_size();
        let t_pos = self.get_pos();
        let o_size = other.get_size();
        let o_pos = other.get_pos();

        if t_pos.0 < o_pos.0 + o_size.0
            && t_pos.0 + t_size.0 > o_pos.0
            && t_pos.1 > o_pos.1 + o_size.1
            && t_pos.1 + t_size.1 < o_pos.1
        {
            return true;
        }

        false
    }

    fn get_pos(&self) -> Point;
    fn get_size(&self) -> Dimensions;
}
