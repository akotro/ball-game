use bevy::prelude::*;
use rand::random;

pub fn spawn_entity<T: Component>(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_path: &str,
    position: Vec2,
    component: T,
) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(position.x, position.y, 0.0),
            texture: asset_server.load(texture_path),
            ..default()
        },
        component,
    ));
}

pub fn despawn_single_entity<T: Component>(
    commands: &mut Commands,
    query: &Query<Entity, With<T>>,
) {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).despawn();
    }
}

pub fn despawn_entities<T: Component>(commands: &mut Commands, query: &Query<Entity, With<T>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn get_player_position_grid(
    window_dimensions: Vec2,
    cell_size: f32,
    player_translation: Vec2,
) -> Vec<Vec<bool>> {
    let rows = (window_dimensions.y / cell_size).round() as usize;
    let columns = (window_dimensions.x / cell_size).round() as usize;

    let mut grid = vec![vec![false; columns]; rows];

    let player_cell_x = (player_translation.x / cell_size).floor() as usize;
    let player_cell_y = (player_translation.y / cell_size).floor() as usize;
    // println!("Player cell: {}, {}", player_cell_x, player_cell_y);
    grid[player_cell_y][player_cell_x] = true;

    grid
}

pub fn find_available_position(grid: &Vec<Vec<bool>>, cell_size: f32) -> Vec2 {
    // NOTE: Get cell strictly inside the window
    let rows = grid.len();
    let columns = grid[0].len();

    loop {
        let entity_cell_x = (random::<f32>() * columns as f32).floor() as usize;
        let entity_cell_y = (random::<f32>() * rows as f32).floor() as usize;

        if !grid[entity_cell_y][entity_cell_x] && !is_adjacent(grid, entity_cell_x, entity_cell_y) {
            let x = entity_cell_x as f32 * cell_size + cell_size / 2.0;
            let y = entity_cell_y as f32 * cell_size + cell_size / 2.0;
            return Vec2::new(x, y);
        }
    }
}

fn is_adjacent(grid: &Vec<Vec<bool>>, x: usize, y: usize) -> bool {
    let rows = grid.len();
    let columns = grid[0].len();

    for row_offset in -1..=1 {
        for col_offset in -1..=1 {
            if row_offset == 0 && col_offset == 0 {
                continue;
            }

            let row_index = y as isize + row_offset;
            let col_index = x as isize + col_offset;

            if row_index >= 0
                && row_index < rows as isize
                && col_index >= 0
                && col_index < columns as isize
                && grid[row_index as usize][col_index as usize]
            {
                return true;
            }
        }
    }

    false
}
