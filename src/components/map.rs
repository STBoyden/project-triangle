use super::entity::Entity;
use super::object::Object;
use ptgui::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Map {
    pub map_name: String,
    pub spawn_point: Point,
    pub objects: Vec<Object>,
    pub entities: Vec<Entity>,
}
