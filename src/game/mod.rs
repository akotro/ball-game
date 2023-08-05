pub mod enemy;
pub mod player;
pub mod score;
pub mod star;
mod systems;
mod ui;

use bevy::prelude::*;

use crate::{events::GameOverEvent, AppState};
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use systems::*;
use ui::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // NOTE: Events
            .add_event::<GameOverEvent>()
            // NOTE: States
            .add_state::<SimulationState>()
            // NOTE: Enter state systems
            .add_system(pause_simulation.in_schedule(OnEnter(AppState::Game)))
            // NOTE: Plugins
            .add_plugin(EnemyPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(ScorePlugin)
            .add_plugin(StarPlugin)
            .add_plugin(GameUIPlugin)
            // NOTE: Systems
            .add_system(toggle_simulation.run_if(in_state(AppState::Game)))
            // NOTE: Exit state systems
            .add_system(resume_simulation.in_schedule(OnExit(AppState::Game)));
    }
}

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
