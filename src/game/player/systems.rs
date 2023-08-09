use bevy::{prelude::*, window::PrimaryWindow};
use bevy_prototype_debug_lines::DebugLines;

use super::{components::*, resources::PlayerPositionGrid};
use crate::{
    enemy::{components::Enemy, ENEMY_SIZE},
    events::{GameOverEvent, SpawnedPlayerEvent},
    game::CELL_SIZE,
    helpers::{despawn_single_entity, get_player_position_grid, spawn_entity},
    score::resources::Score,
    star::{components::Star, STAR_SIZE},
};

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0; // This is the player sprite size.

pub fn spawn_player(
    mut commands: Commands,
    mut spawned_player_event_writer: EventWriter<SpawnedPlayerEvent>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let player_position = Vec2::new(window.width() / 2.0, window.height() / 2.0);
    spawn_entity(
        &mut commands,
        &asset_server,
        "sprites/ball_blue_large.png",
        player_position,
        Player {},
    );

    spawned_player_event_writer.send(SpawnedPlayerEvent { player_position });

    println!("SpawnedPlayerEvent sent!");
}

pub fn generate_player_position_grid(
    mut spawned_player_event_reader: EventReader<SpawnedPlayerEvent>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut player_grid: ResMut<PlayerPositionGrid>,
) {
    let window = window_query.get_single().unwrap();
    let window_dimensions = Vec2::new(window.width(), window.height());

    for event in spawned_player_event_reader.iter() {
        player_grid.grid =
            get_player_position_grid(window_dimensions, CELL_SIZE, event.player_position);
        println!("Player position grid generated!");
    }
}

pub fn update_player_position_grid(
    player_query: Query<&Transform, With<Player>>,
    mut player_grid: ResMut<PlayerPositionGrid>,
) {
    for row in player_grid.grid.iter_mut() {
        row.fill(false);
    }

    let player_transform = player_query.get_single().unwrap();
    let player_translation = player_transform.translation;

    let player_cell_x = (player_translation.x / CELL_SIZE).floor() as usize;
    let player_cell_y = (player_translation.y / CELL_SIZE).floor() as usize;
    println!("Updated player cell: {}, {}", player_cell_x, player_cell_y);
    player_grid.grid[player_cell_y][player_cell_x] = true;
}

pub fn draw_player_position_grid_lines(
    player_grid: ResMut<PlayerPositionGrid>,
    mut lines: ResMut<DebugLines>,
) {
    let rows = player_grid.grid.len();
    let columns = player_grid.grid[0].len();

    for row in 0..=rows {
        let y = row as f32 * CELL_SIZE;
        let start = Vec3::new(0.0, y, 0.0);
        let end = Vec3::new(columns as f32 * CELL_SIZE, y, 0.0);
        lines.line(start, end, 0.0);
    }

    for column in 0..=columns {
        let x = column as f32 * CELL_SIZE;
        let start = Vec3::new(x, 0.0, 0.0);
        let end = Vec3::new(x, rows as f32 * CELL_SIZE, 0.0);
        lines.line(start, end, 0.0);
    }

    if player_grid.is_changed() {
        for row in 0..rows {
            for column in 0..columns {
                if player_grid.grid[row][column] {
                    let x = column as f32 * CELL_SIZE;
                    let y = row as f32 * CELL_SIZE;
                    let top_left = Vec3::new(x, y, 0.0);
                    let top_right = Vec3::new(x + CELL_SIZE, y, 0.0);
                    let bottom_left = Vec3::new(x, y + CELL_SIZE, 0.0);
                    let bottom_right = Vec3::new(x + CELL_SIZE, y + CELL_SIZE, 0.0);
                    lines.line_colored(top_left, top_right, 0.0, Color::RED);
                    lines.line_colored(top_right, bottom_right, 0.0, Color::RED);
                    lines.line_colored(bottom_right, bottom_left, 0.0, Color::RED);
                    lines.line_colored(bottom_left, top_left, 0.0, Color::RED);
                }
            }
        }
    }
}

pub fn despawn_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    despawn_single_entity(&mut commands, &player_query);
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_player_size = PLAYER_SIZE / 2.0; // 32.0
        let x_min = 0.0 + half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = 0.0 + half_player_size;
        let y_max = window.height() - half_player_size;

        let mut translation = player_transform.translation;

        // Bound the player x position
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }
        // Bound the players y position.
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        player_transform.translation = translation;
    }
}

pub fn enemy_hit_player(
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<GameOverEvent>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    score: Res<Score>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE / 2.0;

            if distance < player_radius + enemy_radius {
                let sound_effect = asset_server.load("audio/explosionCrunch_000.ogg");
                audio.play(sound_effect);
                commands.entity(player_entity).despawn();

                game_over_event_writer.send(GameOverEvent {
                    final_score: score.value,
                });
                println!(
                    "Enemy hit player! GameOverEvent sent with score: {}!",
                    score.value
                );
            }
        }
    }
}

pub fn player_hit_star(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut score: ResMut<Score>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (star_entity, star_transform) in star_query.iter() {
            let distance = player_transform
                .translation
                .distance(star_transform.translation);

            if distance < PLAYER_SIZE / 2.0 + STAR_SIZE / 2.0 {
                score.value += 1;
                println!("Player hit star!");
                let sound_effect = asset_server.load("audio/laserLarge_000.ogg");
                audio.play(sound_effect);
                commands.entity(star_entity).despawn();
            }
        }
    }
}
