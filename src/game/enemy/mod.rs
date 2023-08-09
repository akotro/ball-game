pub mod components;
pub mod resources;
mod systems;

use bevy::prelude::*;

use self::{resources::*, systems::*};
use super::{
    player::systems::generate_player_position_grid, SimulationState, SpawnOthersSystemSet,
};
use crate::AppState;

pub const NUMBER_OF_ENEMIES: usize = 4;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0; // This is the enemy sprite size.

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            // NOTE: Enter state systems
            .add_system(
                spawn_enemies
                    .after(generate_player_position_grid)
                    .in_set(SpawnOthersSystemSet)
                    .in_schedule(OnEnter(AppState::Game)),
            )
            // NOTE: Systems
            .add_systems(
                (
                    enemy_movement,
                    update_enemy_direction,
                    confine_enemy_movement,
                    tick_enemy_spawn_timer,
                    spawn_enemies_over_time,
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            // NOTE: Exit state systems
            .add_system(despawn_enemies.in_schedule(OnExit(AppState::Game)));
    }
}
