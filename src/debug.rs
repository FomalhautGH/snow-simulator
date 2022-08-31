use bevy::prelude::*;
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};
use bevy_rapier2d::render::RapierDebugRenderPlugin;
use hik::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugin(WorldInspectorPlugin::new())
                .register_inspectable::<Snow>()
                .add_plugin(RapierDebugRenderPlugin::default());
        }
    }
}
