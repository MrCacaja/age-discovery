use bevy::asset::AssetServer;
use bevy::ecs::component::Component;
use bevy::ecs::bundle::Bundle;
use bevy::prelude::Res;
use bevy::sprite::SpriteBundle;
use crate::{Commands, default, Name, Person};
use crate::game::general::living::Living;

#[derive(Default, Component)]
pub struct Player;

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    player: Player,
    person: Person,
    living: Living,
    name: Name,

    #[bundle]
    sprite: SpriteBundle
}

//TODO: função temporária, deve ser modularizada
pub fn add_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(PlayerBundle {
        name: Name("player".to_string()),
        sprite: SpriteBundle {
            texture: asset_server.load("elf/texture.png"),
            ..default()
        },
        ..default()
    });
}
