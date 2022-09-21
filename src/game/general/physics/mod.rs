use bevy::math::Vec3;
use bevy::ecs::component::Component;
use bevy::prelude::{Query, Res, TextureAtlasSprite, Transform};
use bevy::ecs::bundle::Bundle;
use bevy::time::Time;
use bevy_inspector_egui::Inspectable;
use crate::game::general::MultipleSided;

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

pub fn update_sprites(mut transforms: Query<(&mut Transform, &Physical, Option<&MultipleSided>, Option<&mut TextureAtlasSprite>, Option<&SelfPhysical>)>) {
    for (mut transform, physical, multiple_sided, atlas_sprite, self_physical) in transforms.iter_mut() {
        transform.translation = physical.position;

        if let Some(_) = multiple_sided {
            if let Some(mut atlas_sprite) = atlas_sprite {
                update_atlas_sprites(physical, self_physical, &mut atlas_sprite);
            } else {
                panic!("Multiple sided without atlas_sprite");
            }
        }
    }
}

fn update_atlas_sprites(physical: &Physical, self_physical: Option<&SelfPhysical>, atlas_sprite: &mut TextureAtlasSprite) {
    let mut direction = physical.direction;

    if let Some(self_physical) = self_physical {
        if physical.acceleration < self_physical.speed {
            direction = self_physical.direction;
        }
    }

    direction.x =  f32::trunc(direction.x  * 100.0) / 100.0;
    direction.y =  f32::trunc(direction.y  * 100.0) / 100.0;

    if direction.x > 0. {
        atlas_sprite.index = 0;
        atlas_sprite.flip_x = false;
    }
    else if direction.x < 0. {
        atlas_sprite.index = 0;
        atlas_sprite.flip_x = true;
    }
    else if direction.y < 0. {
        atlas_sprite.index = 1;
        atlas_sprite.flip_x = false;
    }
    else if direction.y > 0. {
        atlas_sprite.index = 2;
        atlas_sprite.flip_x = false;
    }
}

pub fn direction_react(time: Res<Time>, mut entities: Query<(&mut Physical, Option<&SelfPhysical>)>) {
    for (mut physical, self_physical) in entities.iter_mut() {
        if let Some(self_physical) = self_physical {
            if self_physical.speed > physical.acceleration {
                physical.position += self_physical.direction * time.delta_seconds() * self_physical.speed;
            }
        }

        physical.direction = physical.direction.normalize_or_zero();
        let direction = physical.direction;
        let acceleration = physical.acceleration;

        physical.position += direction * time.delta_seconds() * acceleration;
        physical.acceleration = (physical.acceleration - physical.weight).clamp(0., f32::MAX);
    }
}