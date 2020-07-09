use crate::types::*;

pub mod gui_button;
pub mod gui_component;
pub mod gui_cursor;

fn is_inside(position: Point, dimensions: &Dimensions, mouse_position: &Point) -> bool {
    let rect_points = vec![
        position,                                               // top left (A)
        (position.0 + dimensions.0, position.1),                // top right (B)
        (position.0, position.1 + dimensions.1),                // bottom left (C)
        (position.0 + dimensions.0, position.1 + dimensions.1), // bottom right (D)
    ];

    if (mouse_position.0 > rect_points[0].0 && mouse_position.0 > rect_points[2].0) && // X.0 > A.0 || C.0
        (mouse_position.0 < rect_points[1].0 && mouse_position.0 < rect_points[3].0) && // X.0 < B.0 || D.0
        (mouse_position.1 > rect_points[0].1 && mouse_position.1 > rect_points[1].1) && // X.1 > A.1 || B.1
        (mouse_position.1 < rect_points[2].1 && mouse_position.1 < rect_points[3].1)
    {
        // X.1 < C.1 || D.1

        return true;
    }

    false
}
