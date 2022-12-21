pub mod player;

use bevy::ecs::component::Component;
use bevy::ecs::bundle::Bundle;
use bevy_ecs_ldtk::prelude::*;
use bevy::sprite::SpriteSheetBundle;
use crate::game::general::MultipleSided;
use crate::game::general::physics::SelfPhysicalBundle;
use crate::Name;

#[derive(Default, Component)]
pub struct Living;

#[derive(Default, Component)]
pub struct Person;

#[derive(LdtkEntity, Bundle, Default)]
pub struct PersonBundle {
    pub person: Person,
    pub living: Living,
    pub name: Name,
    pub multiple_sided: MultipleSided,

    #[sprite_sheet_bundle]
    #[bundle]
    pub sprite: SpriteSheetBundle,
    #[ldtk_entity]
    #[bundle]
    pub self_physical: SelfPhysicalBundle
}
