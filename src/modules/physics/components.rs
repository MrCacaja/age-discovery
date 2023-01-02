use bevy::ecs::component::Component;
use bevy::ecs::bundle::Bundle;
use bevy::ecs::prelude::Resource;
use bevy::math::{Vec2, Vec3};
use bevy_inspector_egui::Inspectable;
use bevy_ecs_ldtk::{EntityInstance, LdtkEntity};
use crate::{default, Timer};

#[derive(Inspectable, Debug)]
pub enum Side { BOTTOM, LEFT, RIGHT, TOP }

impl Default for Side {
    fn default() -> Self {
        Side::BOTTOM
    }
}

#[derive(Inspectable, Debug)]
pub enum MovementState { IDLE, WALK, DRAG }

impl Default for MovementState {
    fn default() -> Self {
        MovementState::IDLE
    }
}

#[derive(Default, Component, Inspectable)]
pub struct MultipleSided {
    pub side: Side
}

#[derive(Default, Component, Inspectable)]
pub struct MultipleMovementState {
    pub current_index: usize,
    pub state: MovementState,
    pub used_first: bool
}

#[derive(Default, Resource)]
pub struct MovementSpriteTimer {
    pub timer: Timer
}

#[derive(Default, Component, Inspectable)]
pub struct Physical {
    pub direction: Vec3,
    pub weight: f32,
    pub acceleration: f32,
}

impl From<EntityInstance> for Physical {
    fn from(entity_instance: EntityInstance) -> Physical {
        match entity_instance.identifier.as_str() {
            "Player" => Physical { weight: 2.5, ..default() },
            _ => Physical {..default()}
        }
    }
}

#[derive(Inspectable)]
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
            "Player" => Collider(TransformZone {size: Vec2::new(16., 4.5), offset: Vec2::new(0., -8.)}),
            "Rock" => Collider(TransformZone {size: Vec2::new(28., 20.), offset: Vec2::new(-6., -8.)}),
            _ => Collider {..default()}
        }
    }
}

#[derive(Component, Inspectable)]
pub struct SpriteZone(pub TransformZone);

impl Default for SpriteZone {
    fn default() -> Self {
        Self(TransformZone {size: Vec2::new(16., 16.), offset: Vec2::ZERO})
    }
}

impl From<EntityInstance> for SpriteZone {
    fn from(entity_instance: EntityInstance) -> SpriteZone {
        match entity_instance.identifier.as_str() {
            "Player" => SpriteZone(TransformZone {size: Vec2::new(16., 32.), offset: Vec2::new(0., -8.)}),
            "Rock" => SpriteZone(TransformZone {size: Vec2::new(32., 32.), offset: Vec2::new(-6., -8.)}),
            _ => SpriteZone {..default()}
        }
    }
}

#[derive(Component, Inspectable)]
pub struct SelfPhysical {
    pub direction: Vec3,
    pub speed: f32,
}

impl From<EntityInstance> for SelfPhysical {
    fn from(entity_instance: EntityInstance) -> SelfPhysical {
        match entity_instance.identifier.as_str() {
            "Player" => SelfPhysical { speed: 50., ..default() },
            _ => SelfPhysical {..default()}
        }
    }
}

impl Default for SelfPhysical {
    fn default() -> Self {
        Self {
            direction: Vec3::ZERO,
            speed: 1.,
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