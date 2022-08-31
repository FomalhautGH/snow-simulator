use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;
use hik::*;

pub struct SnowPlugin;

impl Plugin for SnowPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_snow_system)
            .add_system(despawn_snow_system);
    }
}

fn spawn_snow_system(
    mut command: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut snow_meshes: ResMut<SnowMeshes>,
    mut cursor_event_reader: EventReader<CursorMoved>,
) {
    for event in cursor_event_reader.iter() {
        command
            .spawn_bundle(MaterialMesh2dBundle {
                mesh: meshes
                    .add(Mesh::from(shape::Quad {
                        size: Vec2::splat(10.0),
                        ..Default::default()
                    }))
                    .into(),
                material: materials.add(ColorMaterial::from(Color::WHITE)),
                transform: Transform::from_xyz(
                    event.position.x - WIN_WIDTH / 2.0,
                    event.position.y - WIN_HEIGHT / 2.0,
                    0.0,
                ),
                ..Default::default()
            })
            .insert(Snow)
            .insert(RigidBody::Dynamic)
            .insert(Collider::cuboid(5.0, 5.0))
            .insert(Name::new(format!("Snow ({})", snow_meshes.number)));

        snow_meshes.number += 1;

        println!("Number of meshes: {}", snow_meshes.number);
    }
}

fn despawn_snow_system(
    mut command: Commands,
    query: Query<Entity, With<Snow>>,
    mut snow_meshes: ResMut<SnowMeshes>,
    time: Res<Time>,
) {
    snow_meshes.timer.tick(time.delta());

    if snow_meshes.timer.just_finished() {
        for snow_mesh in query.iter() {
            command.entity(snow_mesh).despawn();
            snow_meshes.number -= 1;

            println!("Number of meshes: {}", snow_meshes.number);
            return;
        }
    }
}
