use std::iter::Map;
use bevy::ecs::component::Component;
use bevy::ecs::bundle::Bundle;
use bevy::math::Vec2;
use bevy::prelude::{default, Entity, SpriteSheetBundle};
use bevy_ecs_ldtk::{EntityInstance, LdtkEntity};
use bevy_inspector_egui::Inspectable;
use crate::modules::camera::components::CameraTarget;
use crate::modules::living::consts::{ELVEN_ALLIANCES, ELVEN_ENEMIES, ELVEN_GROUPS, GOBLIN_ALLIANCES, GOBLIN_ENEMIES, GOBLIN_GROUPS};
use crate::modules::physics::components::{Collider, SelfPhysicalBundle, TransformZone};
use crate::modules::physics::sprite_change::components::{MultipleMovementState, MultipleSided, SpriteZone};
use crate::modules::simple::components::Name;
use bevy::prelude::{Reflect, ReflectComponent, TimerMode};
use bevy::time::Timer;

#[derive(Default, Component)]
pub struct Living;

#[derive(Default, Component)]
pub struct Person;

#[derive(Component, Inspectable, Default)]
pub struct AggressionZone(pub TransformZone);

impl From<EntityInstance> for AggressionZone {
    fn from(entity_instance: EntityInstance) -> AggressionZone {
        match entity_instance.identifier.as_str() {
            "Player" | "Elf" => generate_radius(75.),
            "Goblin" => generate_radius(100.),
            _ => generate_radius(50.),
        }
    }
}

fn generate_radius(radius: f32) -> AggressionZone {
    AggressionZone {0: TransformZone { size: Vec2::new(radius * 2., radius * 2.), offset: Vec2::new(-radius, -radius) }}
}

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
            "Golbin" => Relationships {
                alliances: Map::collect(GOBLIN_ALLIANCES.to_vec().into_iter().map(String::from)),
                enemies: Map::collect(GOBLIN_ENEMIES.to_vec().into_iter().map(String::from)),
                groups: Map::collect(GOBLIN_GROUPS.to_vec().into_iter().map(String::from))
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
    pub position: Vec2,
    pub target: Option<Entity>
}

impl Default for Desire {
    fn default() -> Self {
        Desire {
            update_timer: Timer::from_seconds(2., TimerMode::Repeating),
            intention: Intention::Idle, position: Vec2::ZERO, target: None,
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

    #[from_entity_instance]
    pub aggression_zone: AggressionZone,

    #[ldtk_entity]
    #[bundle]
    pub person: PersonBundle,
}

#[derive(Default, Bundle, LdtkEntity)]
pub struct ElfBundle {
    #[ldtk_entity]
    #[bundle]
    pub mob_bundle: MobBundle,
}

#[derive(Default, Bundle, LdtkEntity)]
pub struct GoblinBundle {
    #[ldtk_entity]
    #[bundle]
    pub mob_bundle: MobBundle,
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