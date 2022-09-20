use bevy::math::Vec3;
use bevy::ecs::component::Component;
use bevy::prelude::{Query, Res, Transform};
use bevy::ecs::bundle::Bundle;
use bevy::time::Time;
use bevy_inspector_egui::Inspectable;

#[derive(Default, Component, Inspectable)]
pub struct Physical {
    pub direction: Vec3,
    pub position: Vec3,
    pub weight: f32,
    pub acceleration: f32,
}

#[derive(Component, Inspectable)]
pub struct SelfPhysical {
    pub direction: Vec3,
    pub speed: f32,
}

impl Default for SelfPhysical {
    fn default() -> Self {
        Self {
            direction: Vec3::ZERO,
            speed: 1.,
        }
    }
}

#[derive(Bundle, Default)]
pub struct SelfPhysicalBundle {
    pub physical: Physical,
    pub self_physical: SelfPhysical,
}

pub fn update_sprites(mut transforms: Query<(&mut Transform, &Physical)>) {
    for (mut transform, physical) in transforms.iter_mut() {
        transform.translation = physical.position;
    }
}

pub fn direction_react(time: Res<Time>, mut entities: Query<(&mut Physical, Option<&SelfPhysical>)>) {
    for (mut physical, self_physical) in entities.iter_mut() {
        let mut acceleration = physical.acceleration;
        let mut direction = physical.direction;

        if let Some(self_physical) = self_physical {
            if direction.length() < 0.1 {
                direction += self_physical.direction;
                acceleration = self_physical.speed;
            }
        }

        physical.position += direction.normalize_or_zero() * time.delta_seconds() * acceleration;
        physical.acceleration = (physical.acceleration - physical.weight).clamp(0., f32::MAX);
        if physical.acceleration <= 0.  {
            physical.direction = Vec3::ZERO;
        }
    }
}