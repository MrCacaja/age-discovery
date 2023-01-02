use bevy::ecs::component::Component;
use bevy::ecs::bundle::Bundle;
use bevy::prelude::SpriteSheetBundle;
use bevy_inspector_egui::Inspectable;
use bevy_ecs_ldtk::LdtkEntity;
use crate::modules::physics::components::{Collider};
use crate::modules::physics::sprite_change::components::SpriteZone;


#[derive(Default, Component, Inspectable)]
pub struct Prop;

#[derive(Default, Bundle, LdtkEntity)]
pub struct PropBundle {
    pub prop: Prop,

    #[from_entity_instance]
    pub collider: Collider,

    #[from_entity_instance]
    pub sprite_zone: SpriteZone,

    #[sprite_sheet_bundle]
    #[bundle]
    pub sprite: SpriteSheetBundle,
}


#[derive(Default, Bundle, LdtkEntity)]
pub struct TreeStumpBundle {
    #[ldtk_entity]
    #[bundle]
    pub prop_bundle: PropBundle,
}


#[derive(Default, Bundle, LdtkEntity)]
pub struct RockBundle {
    #[ldtk_entity]
    #[bundle]
    pub prop_bundle: PropBundle,
}