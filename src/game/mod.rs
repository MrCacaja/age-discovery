use bevy::app::Plugin;
use bevy::asset::AssetServer;
use bevy::input::Input;
use bevy::math::Vec3;
use bevy::prelude::{Camera, KeyCode, Query, Res, Transform, With, Without};
use bevy::ecs::component::Component;
use bevy_ecs_ldtk::LdtkWorldBundle;
use crate::{App, Camera2dBundle, Commands, default, Name, OrthographicProjection, Physical, ScalingMode};
use crate::game::general::living::player::Player;
use crate::game::general::physics::SelfPhysical;
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};

pub mod general;

pub struct DebugPlugin;

#[derive(Default, Component, Clone)]
pub struct CameraTarget;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(WorldInspectorPlugin::new())
            .register_inspectable::<Name>()
            .register_inspectable::<Physical>()
            .register_inspectable::<SelfPhysical>();
    }
}

pub fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    setup_view(&mut commands);
    setup_tilemap(&mut commands, &asset_server);
}

pub fn read_input(keyboard_input: Res<Input<KeyCode>>, mut player_physics: Query<&mut SelfPhysical, With<Player>>) {
    let mut directions = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::W) {
        directions.y += 1.;
    }
    if keyboard_input.pressed(KeyCode::S) {
        directions.y -= 1.;
    }
    if keyboard_input.pressed(KeyCode::D) {
        directions.x += 1.;
    }
    if keyboard_input.pressed(KeyCode::A) {
        directions.x -= 1.;
    }

    for mut physics in player_physics.iter_mut() {
       physics.direction = directions;
    }
}

pub fn camera_follow(camera_targets: Query<&mut Transform, With<CameraTarget>>, mut cameras: Query<&mut Transform, (With<Camera>, Without<CameraTarget>)>) {
    match camera_targets.get_single() {
        Ok(target) => {
            let mut camera = cameras.get_single_mut().unwrap();

            camera.translation = target.translation;
        }
        _ => {}
    };
}

fn setup_tilemap(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: asset_server.load("tilemaps/simple-forest.ldtk"),
        ..default()
    });
}

fn setup_view(commands: &mut Commands) {
    commands.spawn_bundle(Camera2dBundle {
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::Auto { min_width: 192., min_height: 72. },
            ..default()
        },
        ..default()
    });
}