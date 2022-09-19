use bevy::asset::AssetServer;
use bevy::input::Input;
use bevy::math::Vec3;
use bevy::prelude::{KeyCode, Query, Res, With};
use crate::{Camera2dBundle, Commands, default, OrthographicProjection, ScalingMode};
use crate::game::general::living::player::{add_player, Player};
use crate::game::general::physics::SelfPhysical;

pub mod general;

pub fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    setup_view(&mut commands);
    add_player(commands, asset_server);
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
       physics.self_direction = directions;
    }
}

fn setup_view(commands: &mut Commands) {
    commands.spawn_bundle(Camera2dBundle {
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::Auto { min_width: 64., min_height: 64. },
            ..default()
        },
        ..default()
    });
}