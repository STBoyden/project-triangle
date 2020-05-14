use raylib::prelude::*;

// struct GuiComponent {
// 	pub position: Vector2,
// 	pub dimensions: Vector2
// }


pub trait GuiComponentBehaviour {
    fn draw(&mut self, cursor: &super::gui_cursor::Cursor, draw_handler: &mut RaylibDrawHandle);
    fn is_hovered(&mut self, mouse_position: &Vector2) -> bool;
    fn is_clicked(&mut self, cursor: &super::gui_cursor::Cursor) -> bool;
}