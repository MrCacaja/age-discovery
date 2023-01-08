use bevy::ecs::component::Component;
use bevy::ecs::prelude::Resource;
use bevy::math::Vec2;
use bevy::prelude::{Reflect, ReflectComponent, TimerMode};
use bevy_ecs_ldtk::EntityInstance;
use crate::{default, Timer};
use crate::modules::physics::components::TransformZone;
use crate::modules::physics::sprite_change::consts::MOB_BOTTOM_IDLE_START;
use bevy_inspector_egui::Inspectable;

#[derive(Inspectable, Debug)]
pub enum Side { BOTTOM, LEFT, RIGHT, TOP }

impl Default for Side {
    fn default() -> Self {
        Side::BOTTOM
    }
}

#[derive(Inspectable, Debug, Reflect)]
pub enum MovementState { IDLE, WALK, DRAG }

impl Default for MovementState {
    fn default() -> Self {
        MovementState::IDLE
    }
}

#[derive(Default, Component, Inspectable)]
pub struct MultipleSided {
    pub side: Side,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct MultipleMovementState {
    pub current_index: usize,
    pub state: MovementState,
    pub used_first: bool,
    pub default_duration: f32,
    pub timer: Timer
}

impl Default for MultipleMovementState {
    fn default() -> Self {
        MultipleMovementState {
            timer: Timer::from_seconds(0.2, TimerMode::Repeating), default_duration: 0.2,
            used_first: false, state: MovementState::IDLE, current_index: MOB_BOTTOM_IDLE_START
        }
    }
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
            "Player" | "Elf" => SpriteZone(TransformZone {size: Vec2::new(16., 32.), offset: Vec2::new(0., -8.)}),
            "Rock" => SpriteZone(TransformZone {size: Vec2::new(32., 32.), offset: Vec2::new(-6., -8.)}),
            _ => SpriteZone {..default()}
        }
    }
}