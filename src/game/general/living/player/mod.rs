use bevy::ecs::component::Component;
use bevy::ecs::bundle::Bundle;
use bevy_ecs_ldtk::prelude::*;
use crate::game::CameraTarget;
use crate::game::general::living::PersonBundle;

#[derive(Default, Component)]
pub struct Player;

#[derive(Bundle, Default, LdtkEntity)]
pub struct PlayerBundle {
    pub player: Player,
    pub camera_target: CameraTarget,

    #[ldtk_entity]
    #[bundle]
    pub person: PersonBundle,
}