use bevy::input::Input;
use bevy::math::Vec3;
use bevy::prelude::{KeyCode, Query, Res, With};
use crate::modules::living::components::Player;
use crate::modules::physics::components::SelfPhysical;

pub fn read_input(keyboard_input: Res<Input<KeyCode>>, mut player_physics: Query<&mut SelfPhysical, With<Player>>) {
    let mut directions = Vec3::ZERO;
    let sprint = keyboard_input.pressed(KeyCode::LShift);

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
        if physics.direction != directions {
            physics.direction = directions;
        }
        let mut multiplier = 1.;
        if sprint {
            multiplier = 1.8;
        }
        if multiplier != physics.multiplier {
            physics.multiplier = multiplier;
        }
    }
}