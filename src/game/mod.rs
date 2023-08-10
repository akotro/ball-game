pub mod enemy;
pub mod player;
pub mod score;
pub mod star;
mod systems;
mod ui;

use bevy::prelude::*;

use crate::{events::*, AppState};
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use systems::*;
use ui::*;

// use self::player::systems::draw_player_position_grid_lines;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct SpawnPlayerSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct SpawnOthersSystemSet;

pub const CELL_SIZE: f32 = 64.0;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(SpawnPlayerSystemSet.before(SpawnOthersSystemSet))
            // NOTE: Events
            .add_event::<GameOverEvent>()
            .add_event::<SpawnedPlayerEvent>()
            // NOTE: States
            .add_state::<SimulationState>()
            // NOTE: Enter state systems
            // .add_system(pause_simulation.in_schedule(OnEnter(AppState::Game)))
            // NOTE: Plugins
            .add_plugin(EnemyPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(ScorePlugin)
            .add_plugin(StarPlugin)
            .add_plugin(GameUIPlugin)
            // NOTE: Systems
            .add_system(toggle_simulation.run_if(in_state(AppState::Game)))
            // .add_system(draw_player_position_grid_lines.run_if(in_state(AppState::Game)))
            // NOTE: Exit state systems
            // TODO: This probably should not be the case
            .add_system(resume_simulation.in_schedule(OnExit(AppState::Game)));
    }
}

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
