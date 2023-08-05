pub mod enemy;
pub mod player;
pub mod score;
pub mod star;
mod systems;

use crate::{events::*, AppState};
use bevy::prelude::*;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use systems::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SimulationState>()
            .add_event::<GameOverEvent>()
            .add_plugin(EnemyPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(ScorePlugin)
            .add_plugin(StarPlugin)
            .add_system(toggle_simulation.run_if(in_state(AppState::Game)));
    }
}

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum SimulationState {
    #[default]
    Paused,
    Running,
}
