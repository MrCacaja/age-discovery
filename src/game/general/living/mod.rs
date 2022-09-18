pub mod player;

use bevy::ecs::component::Component;
use bevy::ecs::bundle::Bundle;
use bevy::sprite::SpriteBundle;
use crate::Name;

#[derive(Default, Component)]
pub struct Living;

#[derive(Default, Component)]
pub struct Person;

#[derive(Bundle, Default)]
pub struct PersonBundle {
    pub person: Person,
    pub living: Living,
    pub name: Name,

    #[bundle]
    pub sprite: SpriteBundle
}
