use std::time::Duration;

use bevy::{prelude::*, window::PresentMode};
use bevy_rapier2d::prelude::*;
use hik::*;

mod camera;
mod debug;
mod ground;
mod snow;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(WindowDescriptor {
            title: "Snow".to_string(),
            width: WIN_WIDTH,
            height: WIN_HEIGHT,
            resizable: false,
            present_mode: PresentMode::AutoVsync,
            ..Default::default()
        })
        .insert_resource(Msaa::default()) // Rapier
        .insert_resource(SnowMeshes {
            number: 0,
            timer: Timer::new(Duration::from_millis(300), true),
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default()) // Rapier
        .add_plugin(camera::CameraPlugin)
        .add_plugin(debug::DebugPlugin)
        .add_plugin(ground::GroundPlugin)
        .add_plugin(snow::SnowPlugin)
        .run();
}
