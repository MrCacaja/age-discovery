use bevy::asset::AssetServer;
use bevy::ecs::component::Component;
use bevy::ecs::bundle::Bundle;
use bevy::prelude::Res;
use bevy::sprite::SpriteBundle;
use crate::{Commands, default, Name};
use crate::game::general::living::PersonBundle;

#[derive(Default, Component)]
pub struct Player;

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    pub player: Player,

    #[bundle]
    pub person: PersonBundle,
}

//TODO: função temporária, deve ser modularizada
pub fn add_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(PlayerBundle {
        person: PersonBundle {
            name: Name("player".to_string()),
            sprite: SpriteBundle {
                texture: asset_server.load("elf/texture.png"),
                ..default()
            },
            ..default()
        },
        ..default()
    });
}
