pub mod player;

use bevy::ecs::component::Component;
use bevy::ecs::bundle::Bundle;
use bevy::sprite::SpriteSheetBundle;
use crate::game::general::MultipleSided;
use crate::game::general::physics::SelfPhysicalBundle;
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
    pub multiple_sided: MultipleSided,

    #[bundle]
    pub sprite: SpriteSheetBundle,
    #[bundle]
    pub self_physical: SelfPhysicalBundle
}
