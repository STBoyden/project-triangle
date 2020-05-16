use raylib::prelude::*;

use super::gui_cursor::Cursor;

// struct GuiComponent {
// 	pub position: Vector2,
// 	pub dimensions: Vector2
// }


pub trait GuiComponentBehaviour {
    fn draw(&mut self, cursor: &Cursor, draw_handler: &mut RaylibDrawHandle);
    fn is_hovered(&mut self, mouse_position: &Vector2) -> bool;
    fn is_clicked(&mut self, cursor: &Cursor) -> bool;
}
