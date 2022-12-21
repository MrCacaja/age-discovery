mod game;

use bevy::app::{App};
use bevy::DefaultPlugins;
use bevy::prelude::{Camera2dBundle, Commands, OrthographicProjection};
use bevy::render::camera::ScalingMode;
use bevy::render::texture::ImageSettings;
use bevy::utils::default;
use bevy_ecs_ldtk::{LdtkPlugin, LevelSelection};
use bevy_ecs_ldtk::app::RegisterLdtkObjects;
use crate::game::general::Name;
use crate::game::{camera_follow, DebugPlugin, read_input, setup_game};
use crate::game::general::living::player::PlayerBundle;
use crate::game::general::physics::{direction_react, Physical, update_sprites};

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .add_plugin(DebugPlugin)
        .insert_resource(LevelSelection::Index(0))
        .register_ldtk_entity::<PlayerBundle>("Player")
        .add_startup_system(setup_game)
        .add_system(read_input)
        .add_system(camera_follow)
        .add_system(direction_react)
        .add_system(update_sprites)
        .run();
}
