use bevy::prelude::Vec2;

#[derive(Debug)]
pub struct GameOverEvent {
    pub final_score: u32,
}

#[derive(Debug)]
pub struct SpawnedPlayerEvent {
    pub player_position: Vec2,
}
