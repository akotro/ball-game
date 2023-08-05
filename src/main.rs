pub mod events;
mod game;
pub mod helpers;
mod main_menu;
pub mod systems;

use bevy::prelude::*;
use game::*;
use main_menu::*;
use systems::*;

fn main() {
    App::new()
        // NOTE: Bevy plugins
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Ball Game".to_string(),
                resolution: (1280., 720.).into(),
                // NOTE: Bind to canvas included in `index.html`
                canvas: Some("#bevy".to_owned()),
                // NOTE: Tells wasm not to override default event handling, like F5 and Ctrl+R
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
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
        // HACK: This is a hack to make sure that the game_over_menu's score is updated.
        // Possibly occurs because the GameOverEvent is read both here as well as in
        // game_over_menu's update_final_score_text on the same frame and thus
        // the event is lost in update_final_score_text. Need to find a better solution.
        .add_system(handle_game_over_event.in_base_set(CoreSet::PostUpdate))
        .run();
}

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
