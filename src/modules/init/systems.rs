use bevy::app::Plugin;
use bevy::asset::AssetServer;
use bevy::input::Input;
use bevy::math::Vec3;
use bevy::prelude::{Camera, Changed, KeyCode, Query, Res, Transform, With, Without};
use bevy::ecs::component::Component;
use bevy_ecs_ldtk::LdtkWorldBundle;
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};
use bevy_pixel_camera::PixelCameraBundle;
use crate::{App, Commands, default};
use crate::modules::physics::components::{Collider, MultipleMovementState, MultipleSided, Physical, SelfPhysical, SpriteZone};
use crate::modules::simple::components::Name;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(WorldInspectorPlugin::new())
            .register_inspectable::<Name>()
            .register_inspectable::<Physical>()
            .register_inspectable::<SelfPhysical>()
            .register_inspectable::<Collider>()
            .register_inspectable::<SpriteZone>()
            .register_inspectable::<MultipleMovementState>()
            .register_inspectable::<MultipleSided>();
    }
}

pub fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    setup_view(&mut commands);
    setup_tilemap(&mut commands, &asset_server);
}

fn setup_tilemap(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("tilemaps/simple-forest.ldtk"),
        ..default()
    });
}

fn setup_view(commands: &mut Commands) {
    commands.spawn(PixelCameraBundle::from_resolution(192, 108));
}