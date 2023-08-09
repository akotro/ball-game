use bevy::prelude::Resource;

#[derive(Resource, Default)]
pub struct PlayerPositionGrid {
    pub grid: Vec<Vec<bool>>,
}
