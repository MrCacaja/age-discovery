use bevy::asset::{Assets, AssetServer};
use bevy::ecs::component::Component;
use bevy::ecs::bundle::Bundle;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Res, ResMut, SpriteSheetBundle, TextureAtlas};
use crate::{Commands, default, Name};
use crate::game::CameraTarget;
use crate::game::general::living::PersonBundle;
use crate::game::general::physics::{Physical, SelfPhysical, SelfPhysicalBundle};

#[derive(Default, Component)]
pub struct Player;

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    pub player: Player,
    pub camera_target: CameraTarget,

    #[bundle]
    pub person: PersonBundle,
}

//TODO: função temporária, deve ser modularizada
pub fn add_player(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    let texture_handle = asset_server.load("sprites/elf/texture.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 32.0), 3, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn_bundle(PlayerBundle {
        person: PersonBundle {
            name: Name("player".to_string()),
            sprite: SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                ..default()
            },
            self_physical: SelfPhysicalBundle {
                physical: Physical {
                    weight: 2.5,
                    position: Vec3 {
                        z: 500.,
                        ..default()
                    },
                    ..default()
                },
                self_physical: SelfPhysical {
                    speed: 50.,
                    ..default()
                },
                ..default()
            },
            ..default()
        },
        ..default()
    });
}
