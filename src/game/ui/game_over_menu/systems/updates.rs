use bevy::prelude::*;

use crate::{
    events::GameOverEvent,
    game::{
        score::resources::HighScores,
        ui::game_over_menu::components::{FinalScoreText, HighScoreText},
    },
};

pub fn update_final_score_text(
    mut game_over_event_reader: EventReader<GameOverEvent>,
    mut text_query: Query<&mut Text, With<FinalScoreText>>,
) {
    for event in game_over_event_reader.iter() {
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!("Final Score: {}", event.final_score);
        }
    }
}

pub fn update_high_score_text(
    mut game_over_event_reader: EventReader<GameOverEvent>,
    mut text_query: Query<&mut Text, With<HighScoreText>>,
    high_scores: Res<HighScores>,
) {
    for event in game_over_event_reader.iter() {
        for mut text in text_query.iter_mut() {
            // TODO: Refactor HighScores data structure because this is kinda dumb
            if high_scores.is_changed()
                && !high_scores.scores.is_empty()
                && event.final_score == high_scores.scores[high_scores.scores.len() - 1].1
            {
                text.sections[0].value = "New High Score!".to_string();
                text.sections[0].style.color = Color::SEA_GREEN;
            } else {
                text.sections[0].value =
                    format!("High Score: {}", high_scores.scores.last().unwrap().1);
            }
        }
    }
}
