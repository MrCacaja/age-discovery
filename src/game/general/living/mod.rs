pub mod player;

use bevy::ecs::component::Component;
use bevy::ecs::bundle::Bundle;
use bevy_ecs_ldtk::prelude::*;
use bevy::sprite::SpriteSheetBundle;
use crate::game::general::physics::{MultipleMovementState, MultipleSided, SelfPhysicalBundle, SpriteZone};
use crate::{Collider, Name};

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
    pub multiple_movement_state: MultipleMovementState,

    #[from_entity_instance]
    pub collider: Collider,

    #[from_entity_instance]
    pub sprite_zone: SpriteZone,

    #[sprite_sheet_bundle]
    #[bundle]
    pub sprite: SpriteSheetBundle,

    #[ldtk_entity]
    #[bundle]
    pub self_physical: SelfPhysicalBundle
}
