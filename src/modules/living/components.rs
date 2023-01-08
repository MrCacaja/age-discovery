use std::iter::Map;
use bevy::ecs::component::Component;
use bevy::ecs::bundle::Bundle;
use bevy::math::Vec2;
use bevy::prelude::{default, SpriteSheetBundle};
use bevy_ecs_ldtk::{EntityInstance, LdtkEntity};
use bevy_inspector_egui::Inspectable;
use crate::modules::camera::components::CameraTarget;
use crate::modules::living::consts::{ELVEN_ALLIANCES, ELVEN_ENEMIES, ELVEN_GROUPS};
use crate::modules::physics::components::{Collider, SelfPhysicalBundle};
use crate::modules::physics::sprite_change::components::{MultipleMovementState, MultipleSided, SpriteZone};
use crate::modules::simple::components::Name;
use bevy::prelude::{Reflect, ReflectComponent, TimerMode};
use bevy::time::Timer;

#[derive(Default, Component)]
pub struct Living;

#[derive(Default, Component)]
pub struct Person;

#[derive(Default, Component, Inspectable)]
pub struct Relationships {
    pub alliances: Vec<String>,
    pub enemies: Vec<String>,
    pub groups: Vec<String>
}

impl From<EntityInstance> for Relationships {
    fn from(entity_instance: EntityInstance) -> Relationships {
        match entity_instance.identifier.as_str() {
            "Player" | "Elf" => Relationships {
                alliances: Map::collect(ELVEN_ALLIANCES.to_vec().into_iter().map(String::from)),
                enemies: Map::collect(ELVEN_ENEMIES.to_vec().into_iter().map(String::from)),
                groups: Map::collect(ELVEN_GROUPS.to_vec().into_iter().map(String::from))
            },
            _ => Relationships {..default()}
        }
    }

}

#[derive(Inspectable, Debug, Reflect)]
pub enum Intention { WalkTo, Idle, Attack }

impl Default for Intention {
    fn default() -> Self {
        Intention::Idle
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Desire {
    pub update_timer: Timer,
    pub intention: Intention,
    pub position: Vec2
}

impl Default for Desire {
    fn default() -> Self {
        Desire {
            update_timer: Timer::from_seconds(2., TimerMode::Repeating),
            intention: Intention::Idle,
            position: Vec2::ZERO
        }
    }
}

#[derive(LdtkEntity, Bundle, Default)]
pub struct PersonBundle {
    pub person: Person,
    pub living: Living,
    pub name: Name,
    pub multiple_sided: MultipleSided,
    pub multiple_movement_state: MultipleMovementState,

    #[from_entity_instance]
    pub relationships: Relationships,

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
pub struct Mob;

#[derive(Bundle, Default, LdtkEntity)]
pub struct MobBundle {
    pub player: Mob,
    pub desire: Desire,

    #[ldtk_entity]
    #[bundle]
    pub person: PersonBundle,
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