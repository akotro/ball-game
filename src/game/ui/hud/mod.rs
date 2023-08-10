mod components;
mod resources;
mod styles;
mod systems;

use bevy::prelude::*;

use self::systems::{layout::*, updates::*};
use crate::{
    game::ui::hud::systems::updates::{update_enemy_text, update_score_text},
    AppState,
};

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter Systems
            .add_systems(
                (insert_countdown_timer, spawn_hud).in_schedule(OnEnter(AppState::Startup)),
            )
            // Systems
            .add_systems(
                (tick_countdown_timer, update_countdown_text).in_set(OnUpdate(AppState::Startup)),
            )
            .add_systems((update_score_text, update_enemy_text).in_set(OnUpdate(AppState::Game)))
            // OnExit Systems
            .add_systems(
                (remove_countdown_timer, despawn_countdown_hud)
                    .in_schedule(OnExit(AppState::Startup)),
            )
            .add_system(despawn_hud.in_schedule(OnExit(AppState::Game)));
    }
}
