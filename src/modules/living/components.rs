use bevy::ecs::component::Component;
use bevy::ecs::bundle::Bundle;
use bevy::prelude::SpriteSheetBundle;
use bevy_inspector_egui::Inspectable;
use bevy_ecs_ldtk::LdtkEntity;
use crate::modules::camera::components::CameraTarget;
use crate::modules::physics::components::{Collider, MultipleMovementState, MultipleSided, SelfPhysicalBundle, SpriteZone};
use crate::modules::simple::components::Name;

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