use bevy::ecs::component::Component;
use bevy::ecs::bundle::Bundle;
use bevy::prelude::SpriteSheetBundle;
use bevy_inspector_egui::Inspectable;
use bevy_ecs_ldtk::{EntityInstance, GridCoords, LdtkEntity, LdtkIntCell, LdtkLevel};
use crate::Collider;

#[derive(Default, Component, Inspectable)]
pub struct Prop;

#[derive(Default, Component, Bundle, LdtkEntity)]
pub struct PropBundle {
    pub prop: Prop,

    #[from_entity_instance]
    pub collider: Collider,

    #[sprite_sheet_bundle]
    #[bundle]
    pub sprite: SpriteSheetBundle,
}


#[derive(Default, Component, Bundle, LdtkEntity)]
pub struct TreeStumpBundle {
    #[ldtk_entity]
    #[bundle]
    pub prop_bundle: PropBundle,
}


#[derive(Default, Component, Bundle, LdtkEntity)]
pub struct RockBundle {
    #[ldtk_entity]
    #[bundle]
    pub prop_bundle: PropBundle,
}

