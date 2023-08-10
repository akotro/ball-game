use bevy::{
    prelude::Resource,
    time::{Timer, TimerMode},
};

pub const COUNTDOWN_TIME: f32 = 4.0;

#[derive(Resource)]
pub struct StartCountdownTimer {
    pub timer: Timer,
}

impl Default for StartCountdownTimer {
    fn default() -> Self {
        StartCountdownTimer {
            timer: Timer::from_seconds(COUNTDOWN_TIME, TimerMode::Once),
        }
    }
}
