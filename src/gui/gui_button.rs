use raylib::prelude::*;

use super::gui_component::GuiComponentBehaviour;
use super::gui_cursor::Cursor;
use super::{gui_component::GuiComponentBehaviour, gui_cursor::Cursor};
use crate::components::game::GameStates;

pub struct Button<'a> {
    pub position: Vector2,
    pub dimensions: Vector2,
    pub button_text: String,
    action: &'a mut bool,
    hovered: bool,
    normal_colour: Color,
    hover_colour: Color,
    current_colour: Color,
    text_colour: Color,
}

static mut DEFAULT_ACTION: bool = false;

unsafe fn default_action<'a>() -> &'a mut bool { &mut DEFAULT_ACTION }

impl<'a> Button<'a> {
    pub fn new(button_text: String, position: Vector2, dimensions: Vector2,
               action: Option<&'a mut bool>, normal_colour: Color, hover_colour: Color,
               text_colour: Option<Color>) -> Self {
        Button {
            position,
            dimensions,
            button_text,
            action: unsafe { action.unwrap_or(default_action()) },
            hovered: false,
            normal_colour,
            hover_colour,
            current_colour: normal_colour,
            text_colour: text_colour.unwrap_or(Color::WHITE),
        }
    }
}

impl GuiComponentBehaviour for Button<'_> {
    fn draw(&mut self, cursor: &Cursor, draw_handler: &mut RaylibDrawHandle) {
        let mouse_position = cursor.position;
        self.is_hovered(&mouse_position);
        self.is_clicked(cursor);

        let text_position = Vector2::new(
            self.position.x + 10_f32,
            self.position.y + self.dimensions.y - 30_f32,
        );

        draw_handler.draw_rectangle_v(self.position, self.dimensions, self.current_colour);
        draw_handler.draw_text(self.button_text.as_ref(), text_position.x as i32,
                               text_position.y as i32, 20, self.text_colour);
    }

    fn is_hovered(&mut self, mouse_position: &Vector2) -> bool {
        self.hovered = super::is_inside(self.position, &self.dimensions, &mouse_position);

        if self.hovered {
            self.current_colour = self.hover_colour;
        } else {
            self.current_colour = self.normal_colour;
        }

        self.hovered
    }

    fn is_clicked(&mut self, cursor: &Cursor) -> bool {
        let is_inside = super::is_inside(self.position, &self.dimensions, &cursor.position);
        let is_clicked = is_inside && cursor.is_clicked;

        if is_clicked {
            match self.action {
                true => *self.action = false,
                _ => *self.action = true,
            }
        }

        is_clicked
    }
}
