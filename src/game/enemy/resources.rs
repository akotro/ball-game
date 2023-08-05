use bevy::{
    prelude::Resource,
    time::{Timer, TimerMode},
};

pub const ENEMY_SPAWN_TIME: f32 = 1.5;

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        EnemySpawnTimer {
            timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}
