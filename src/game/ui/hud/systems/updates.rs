use bevy::prelude::*;

use crate::{
    game::{
        enemy::components::Enemy,
        score::resources::Score,
        ui::hud::{
            components::{CountdownText, EnemyText, ScoreText},
            resources::StartCountdownTimer,
        },
    },
    AppState,
};

pub fn update_score_text(mut text_query: Query<&mut Text, With<ScoreText>>, score: Res<Score>) {
    if score.is_changed() {
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!("{}", score.value);
        }
    }
}

pub fn update_enemy_text(
    mut text_query: Query<&mut Text, With<EnemyText>>,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    let count = enemy_query.iter().count();
    for mut text in text_query.iter_mut() {
        text.sections[0].value = format!("{}", count);
    }
}

pub fn insert_countdown_timer(mut commands: Commands) {
    commands.insert_resource(StartCountdownTimer::default());
}

pub fn remove_countdown_timer(mut commands: Commands) {
    commands.remove_resource::<StartCountdownTimer>();
}

pub fn tick_countdown_timer(mut countdown_timer: ResMut<StartCountdownTimer>, time: Res<Time>) {
    countdown_timer.timer.tick(time.delta());
}

pub fn update_countdown_text(
    mut text_query: Query<&mut Text, With<CountdownText>>,
    countdown_timer: Res<StartCountdownTimer>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    let remaining_seconds =
        (countdown_timer.timer.duration() - countdown_timer.timer.elapsed()).as_secs();
    for mut text in text_query.iter_mut() {
        let text_value = match remaining_seconds {
            3 => "3",
            2 => "2",
            1 => "1",
            0 => {
                text.sections[0].style.color = Color::SEA_GREEN;
                "Go!"
            }
            _ => "",
        };

        text.sections[0].value = text_value.to_string();
    }

    if remaining_seconds == 0 {
        app_state_next_state.set(AppState::Game);
        println!("Entered AppState::Game");
    }
}
