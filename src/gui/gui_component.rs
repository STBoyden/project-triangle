use raylib::prelude::*;

use super::gui_cursor::Cursor;
use crate::components::game::GameStates;

pub trait GuiComponentBehaviour {
    fn draw(
        &mut self,
        cursor: &Cursor,
        draw_handler: &mut RaylibDrawHandle,
        state: &mut GameStates,
    );
    fn is_hovered(&mut self, mouse_position: &Vector2) -> bool;
    fn is_clicked(&mut self, cursor: &Cursor, state: &mut GameStates) -> bool;
}
