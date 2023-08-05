pub mod components;
mod systems;

use bevy::prelude::*;

use self::systems::*;
use super::SimulationState;
use crate::AppState;

// NOTE:(akotro) Alternatives of before()/after():
// .add_systems((player_movement, confine_player_movement).chain())
// or
// #[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
// pub enum PlayerSystemSet {
//     Movement,
//     Confinement,
// }
// or
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MovementSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ConfinementSystemSet;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(MovementSystemSet.before(ConfinementSystemSet))
            // NOTE: Enter state systems
            .add_system(spawn_player.in_schedule(OnEnter(AppState::Game)))
            // NOTE: Systems
            .add_system(
                player_movement
                    .in_set(MovementSystemSet)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_system(
                confine_player_movement
                    .in_set(ConfinementSystemSet)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_systems(
                (enemy_hit_player, player_hit_star)
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            // NOTE: Exit state systems
            .add_system(despawn_player.in_schedule(OnExit(AppState::Game)));
    }
}
