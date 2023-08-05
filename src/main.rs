pub mod events;
mod game;
mod main_menu;
pub mod systems;

use bevy::prelude::*;
use game::*;
use main_menu::*;
use systems::*;

fn main() {
    App::new()
        // NOTE: Bevy plugins
        .add_plugins(DefaultPlugins)
        // NOTE: State
        .add_state::<AppState>()
        // NOTE: My plugins
        .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugin)
        // NOTE: Startup systems
        .add_startup_system(spawn_camera)
        // NOTE: Systems
        .add_system(transition_to_game_state)
        .add_system(transition_to_main_menu_state)
        .add_system(exit_game)
        .add_system(handle_game_over_event)
        .run();
}

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
