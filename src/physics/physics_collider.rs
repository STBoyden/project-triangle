use crate::types::*;
use std::cmp::{max, min};

pub trait PhysicsCollider {
    fn is_colliding<T>(&self, other: T) -> bool
    where
        T: PhysicsCollider,
    {
        let t_size = self.get_size();
        let t_pos = self.get_pos();
        let o_size = other.get_size();
        let o_pos = other.get_pos();

        let t_rect = [t_pos.0, t_pos.1, t_pos.0 + t_size.0, t_pos.1 + t_size.1];
        let o_rect = [o_pos.0, o_pos.1, o_pos.0 + o_size.0, o_pos.1 + o_size.1];

        (min(t_rect[2], o_rect[2]) > max(t_rect[0], o_rect[0]))
            && (min(t_rect[3], o_rect[3]) > max(t_rect[1], o_rect[1]))
    }

    fn get_pos(&self) -> Point;
    fn get_size(&self) -> Dimensions;
}
