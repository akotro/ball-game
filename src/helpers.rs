use bevy::prelude::*;

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
