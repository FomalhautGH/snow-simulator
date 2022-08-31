use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use hik::*;

pub struct GroundPlugin;

impl Plugin for GroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_ground_system);
    }
}

fn spawn_ground_system(mut command: Commands) {
    command
        .spawn_bundle(TransformBundle {
            local: Transform::from_xyz(0.0, -370.0, 0.0),
            ..Default::default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(WIN_WIDTH, 10.0))
        .insert(Name::new("Ground"));
}
