use bevy::prelude::*;

use super::{components::*, resources::*, NUMBER_OF_STARS};
use crate::{
    game::{player::resources::PlayerPositionGrid, CELL_SIZE},
    helpers::*,
};

pub fn spawn_stars(
    mut commands: Commands,
    player_position_grid: Res<PlayerPositionGrid>,
    asset_server: Res<AssetServer>,
) {
    for _ in 0..NUMBER_OF_STARS {
        let random_position = find_available_position(&player_position_grid.grid, CELL_SIZE);

        spawn_entity(
            &mut commands,
            &asset_server,
            "sprites/star.png",
            random_position,
            Star {},
        );
    }
}

pub fn despawn_stars(mut commands: Commands, star_query: Query<Entity, With<Star>>) {
    despawn_entities(&mut commands, &star_query);
}

pub fn tick_star_spawn_timer(mut star_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_stars_over_time(
    mut commands: Commands,
    player_position_grid: Res<PlayerPositionGrid>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<StarSpawnTimer>,
) {
    if star_spawn_timer.timer.finished() {
        let random_position = find_available_position(&player_position_grid.grid, CELL_SIZE);

        spawn_entity(
            &mut commands,
            &asset_server,
            "sprites/star.png",
            random_position,
            Star {},
        );
    }
}
