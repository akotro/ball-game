use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

use super::{components::*, resources::*, NUMBER_OF_STARS};
use crate::helpers::*;

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_STARS {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        spawn_entity(
            &mut commands,
            &asset_server,
            "sprites/star.png",
            Vec2::new(random_x, random_y),
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
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<StarSpawnTimer>,
) {
    if star_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        spawn_entity(
            &mut commands,
            &asset_server,
            "sprites/star.png",
            Vec2::new(random_x, random_y),
            Star {},
        );
    }
}
