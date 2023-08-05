pub mod resources;
mod systems;

use bevy::prelude::*;

use self::{resources::*, systems::*};
use crate::AppState;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
            // .init_resource::<Score>()
            .init_resource::<HighScores>()
            // NOTE: Enter state systems
            .add_system(insert_score.in_schedule(OnEnter(AppState::Game)))
            // NOTE: Systems
            .add_system(update_score.run_if(in_state(AppState::Game)))
            .add_system(update_high_scores)
            .add_system(high_scores_updated)
            // NOTE: Exit state systems
            .add_system(remove_score.in_schedule(OnExit(AppState::Game)));
    }
}
