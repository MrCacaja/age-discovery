use bevy::math::Vec3;
use bevy::ecs::component::Component;
use bevy::prelude::{Query, Res, TextureAtlasSprite, Transform};
use bevy::ecs::bundle::Bundle;
use bevy::time::Time;
use bevy_ecs_ldtk::{EntityInstance, LdtkEntity};
use bevy_inspector_egui::Inspectable;
use crate::default;
use crate::game::general::MultipleSided;

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

pub fn update_sprites(mut transforms: Query<(&Physical, Option<&MultipleSided>, Option<&mut TextureAtlasSprite>, Option<&SelfPhysical>)>) {
    for (physical, multiple_sided, atlas_sprite, self_physical) in transforms.iter_mut() {
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

pub fn direction_react(time: Res<Time>, mut entities: Query<(&mut Physical, Option<&SelfPhysical>, Option<&mut Transform>)>) {
    for (mut physical, self_physical, transform) in entities.iter_mut() {
        if let Some(mut transform) = transform {
            if let Some(self_physical) = self_physical {
                if self_physical.speed > physical.acceleration {
                    transform.translation += self_physical.direction * time.delta_seconds() * self_physical.speed;
                }
            }
            physical.direction = physical.direction.normalize_or_zero();
            transform.translation += physical.direction * time.delta_seconds() * physical.acceleration;
            physical.acceleration = (physical.acceleration - physical.weight).clamp(0., f32::MAX);
        } else {
            physical.direction = physical.direction.normalize_or_zero();
            physical.acceleration = (physical.acceleration - physical.weight).clamp(0., f32::MAX);
        }

    }
}