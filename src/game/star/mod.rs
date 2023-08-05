pub mod components;
mod resources;
mod systems;

use self::{resources::*, systems::*};
use super::SimulationState;
use crate::AppState;
use bevy::prelude::*;

pub const NUMBER_OF_STARS: usize = 10;
pub const STAR_SIZE: f32 = 30.0; // This is the star sprite size.

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
            // NOTE: Enter state systems
            .add_system(spawn_stars.in_schedule(OnEnter(AppState::Game)))
            // NOTE: Systems
            .add_systems(
                (tick_star_spawn_timer, spawn_stars_over_time)
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            // NOTE: Exit state systems
            .add_system(despawn_stars.in_schedule(OnExit(AppState::Game)));
    }
}
