use raylib::core::math::Vector2;

pub mod gui_component;
pub mod gui_button;
pub mod gui_component;
pub mod gui_cursor;

fn is_inside(position: Vector2, dimensions: &Vector2, mouse_position: &Vector2) -> bool {
    let rect_points = vec![
        position,                                                                // top left (A)
        Vector2::new(position.x + dimensions.x, position.y),                     // top right (B)
        Vector2::new(position.x, position.y + dimensions.y),                     // bottom left (C)
        Vector2::new(position.x + dimensions.x, position.y + dimensions.y)       // bottom right (D)
    ];

    if (mouse_position.x > rect_points[0].x && mouse_position.x > rect_points[2].x) && // X.x > A.x || C.x
        (mouse_position.x < rect_points[1].x && mouse_position.x < rect_points[3].x) && // X.x < B.x || D.x
        (mouse_position.y > rect_points[0].y && mouse_position.y > rect_points[1].y) && // X.y > A.y || B.y
        (mouse_position.y < rect_points[2].y && mouse_position.y < rect_points[3].y) {  // X.y < C.y || D.y

        return true;
    }

    false
}
