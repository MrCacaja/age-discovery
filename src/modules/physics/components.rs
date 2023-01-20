use bevy::ecs::component::Component;
use bevy::ecs::bundle::Bundle;
use bevy::math::{Vec2, Vec3};
use bevy_inspector_egui::Inspectable;
use bevy_ecs_ldtk::{EntityInstance, LdtkEntity};
use crate::default;

#[derive(Default, Component, Inspectable)]
pub struct Physical {
    pub direction: Vec3,
    pub weight: f32,
    pub acceleration: f32,
}

impl From<EntityInstance> for Physical {
    fn from(entity_instance: EntityInstance) -> Physical {
        match entity_instance.identifier.as_str() {
            "Player" | "Elf" => Physical { weight: 2.5, ..default() },
            "Goblin" => Physical { weight: 1., ..default() },
            _ => Physical {..default()}
        }
    }
}

#[derive(Inspectable, Default)]
pub struct TransformZone {
    pub size: Vec2,
    pub offset: Vec2
}

#[derive(Component, Inspectable)]
pub struct Collider(pub TransformZone);

impl Default for Collider {
    fn default() -> Self {
        Self(TransformZone {size: Vec2::new(16., 10.), offset: Vec2::ZERO})
    }
}

impl From<EntityInstance> for Collider {
    fn from(entity_instance: EntityInstance) -> Collider {
        match entity_instance.identifier.as_str() {
            "Player" | "Elf" => Collider(TransformZone {size: Vec2::new(16., 4.5), offset: Vec2::new(0., -8.)}),
            "Goblin" => Collider(TransformZone {size: Vec2::new(16., 2.25), offset: Vec2::new(0., 0.)}),
            "Rock" => Collider(TransformZone {size: Vec2::new(28., 20.), offset: Vec2::new(-6., -8.)}),
            _ => Collider {..default()}
        }
    }
}

#[derive(Component, Inspectable)]
pub struct SelfPhysical {
    pub direction: Vec3,
    pub speed: f32,
    pub multiplier: f32,
}

impl From<EntityInstance> for SelfPhysical {
    fn from(entity_instance: EntityInstance) -> SelfPhysical {
        match entity_instance.identifier.as_str() {
            "Player" | "Elf" => SelfPhysical { speed: 50., ..default() },
            "Goblin" => SelfPhysical { speed: 75., ..default() },
            _ => SelfPhysical {..default()}
        }
    }
}

impl Default for SelfPhysical {
    fn default() -> Self {
        Self {
            direction: Vec3::ZERO,
            speed: 1.,
            multiplier: 1.,
        }
    }
}

#[derive(Bundle, Default, LdtkEntity)]
pub struct SelfPhysicalBundle {
    #[from_entity_instance]
    pub physical: Physical,
    #[from_entity_instance]
    pub self_physical: SelfPhysical,
}