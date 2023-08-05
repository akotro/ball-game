use bevy::prelude::Resource;

#[derive(Default, Resource)]
pub struct Score {
    pub value: u32,
}

#[derive(Default, Debug, Resource)]
pub struct HighScores {
    pub scores: Vec<(String, u32)>,
}
