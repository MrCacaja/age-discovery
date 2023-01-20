use bevy::app::Plugin;
use bevy::asset::AssetServer;
use bevy::prelude::Res;
use bevy_ecs_ldtk::LdtkWorldBundle;
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};
use bevy_pixel_camera::PixelCameraBundle;
use crate::{App, Commands, default};
use crate::modules::living::components::{AggressionZone, Desire, Relationships};
use crate::modules::physics::components::{Collider, Physical, SelfPhysical};
use crate::modules::physics::sprite_change::components::{MultipleMovementState, MultipleSided, SpriteZone};
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
            .register_inspectable::<MultipleSided>()
            .register_inspectable::<Relationships>()
            .register_inspectable::<AggressionZone>()
            .register_type::<Desire>()
            .register_type::<MultipleMovementState>();
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