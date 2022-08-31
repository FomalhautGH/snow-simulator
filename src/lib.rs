use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

pub const WIN_WIDTH: f32 = 1080.0;
pub const WIN_HEIGHT: f32 = 720.0;

#[derive(Component)]
pub struct Camera2DComponent;

#[derive(Component, Inspectable)]
pub struct Snow;

pub struct SnowMeshes {
    pub number: u64,
    pub timer: Timer,
}
