use bevy::asset::AssetServer;
use bevy::input::Input;
use bevy::math::Vec3;
use bevy::prelude::{KeyCode, Query, Res, ResMut, Transform, With};
use crate::{Camera2dBundle, Commands, default, OrthographicProjection, ScalingMode};
use crate::game::general::living::player::{add_player, Player};

pub mod general;

pub fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    setup_view(&mut commands);
    add_player(commands, asset_server);
}

pub fn read_input(keyboard_input: Res<Input<KeyCode>>, mut player_transforms: Query<&mut Transform, With<Player>>) {
    // TODO: as direções devem vir do jogador, sendo que deve haver um sistema para atualizar a posição
    // de acordo com a direção e o input deve apenas fazer uma força de acordo com a direção e delta
    // time deve ser implementado
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

    for mut player_transform in player_transforms.iter_mut() {
        player_transform.translation += directions;
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