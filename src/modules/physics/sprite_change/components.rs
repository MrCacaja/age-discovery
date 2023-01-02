use bevy::ecs::component::Component;
use bevy::ecs::prelude::Resource;
use bevy::math::Vec2;
use bevy_inspector_egui::Inspectable;
use bevy_ecs_ldtk::EntityInstance;
use crate::{default, Timer};
use crate::modules::physics::components::TransformZone;

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