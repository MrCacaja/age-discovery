use std::collections::{HashMap, HashSet};
use bevy::asset::{Assets, AssetServer, Handle};
use bevy::math::{Vec2, Vec3};
use bevy::ecs::component::Component;
use bevy::prelude::{Added, Changed, Commands, Entity, EventWriter, Or, Query, Res, ResMut, SpriteSheetBundle, TextureAtlas, TextureAtlasSprite, Transform, Without};
use bevy::ecs::bundle::Bundle;
use bevy::hierarchy::{BuildChildren, Parent};
use bevy::prelude::KeyCode::V;
use bevy::sprite::Sprite;
use bevy::time::Time;
use bevy_ecs_ldtk::{EntityInstance, GridCoords, LdtkEntity, LdtkIntCell, LdtkLevel};
use bevy_ecs_ldtk::ldtk::LayerInstance;
use bevy_inspector_egui::Inspectable;
use crate::{default, Name, PlayerBundle};
use crate::game::general::living::PersonBundle;
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
pub struct Collider {
    pub position: Vec2,
    pub size: Vec2,
    pub offset: Vec2
}

impl Default for Collider {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            size: Vec2::new(16., 10.),
            offset: Vec2::ZERO
        }
    }
}

impl From<EntityInstance> for Collider {
    fn from(entity_instance: EntityInstance) -> Collider {
        match entity_instance.identifier.as_str() {
            "Player" => Collider { offset: Vec2::new(0., -8.), size: Vec2::new(16., 4.5), ..default() },
            "Rock" => Collider { offset: Vec2::new(-6., -8.), size: Vec2::new(28., 20.), ..default() },
            _ => Collider {..default()}
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

pub fn direction_react(
    time: Res<Time>,
    mut entities: Query<(Entity, &mut Physical, Option<&SelfPhysical>, &mut Transform, Option<&Collider>)>,
    colliders: Query<(Entity, &Collider)>
) {
    for (entity, mut physical, self_physical, mut transform, collider) in entities.iter_mut() {
        if let Some(collider) = collider {
            let mut direction = physical.direction;
            if let Some(self_physical) = self_physical {
                direction += self_physical.direction;
            }
            if direction.x != 0. || direction.y != 0. {
                let future_collider_pos = Vec2::new(direction.x + collider.position.x, direction.y + collider.position.y);
                if future_collider_pos.x != 0. || future_collider_pos.y != 0. {
                    let mut collided = false;
                    for (target_entity, target_collider) in colliders.iter() {
                        if (target_entity == entity) {
                            continue;
                        }
                        let target_collider_len = target_collider.position + target_collider.size;
                        let collider_len = future_collider_pos + collider.size;
                        collided = !(collider_len.y < target_collider.position.y || future_collider_pos.y > target_collider_len.y || collider_len.x < target_collider.position.x || future_collider_pos.x > target_collider_len.x);
                        if collided {break}
                    }
                    if collided {
                        physical.acceleration = (physical.acceleration - physical.weight).clamp(0., f32::MAX);
                    } else {
                        if let Some(self_physical) = self_physical {
                            if self_physical.speed > physical.acceleration {
                                transform.translation += self_physical.direction * time.delta_seconds() * self_physical.speed;
                            }
                        }

                        physical.direction = physical.direction.normalize_or_zero();
                        transform.translation += physical.direction * time.delta_seconds() * physical.acceleration;
                        physical.acceleration = (physical.acceleration - physical.weight).clamp(0., f32::MAX);
                    }
                }
            }
        } else {
            if let Some(self_physical) = self_physical {
                if self_physical.speed > physical.acceleration {
                    transform.translation += self_physical.direction * time.delta_seconds() * self_physical.speed;
                }
            }

            physical.direction = physical.direction.normalize_or_zero();
            transform.translation += physical.direction * time.delta_seconds() * physical.acceleration;
            physical.acceleration = (physical.acceleration - physical.weight).clamp(0., f32::MAX);
        }
    }
}

pub fn update_collider_pos(mut colliders: Query<(&mut Collider, &Transform, &TextureAtlasSprite)>) {
    for (mut collider, transform, texture) in colliders.iter_mut() {
        collider.position = Vec2::new(transform.translation.x + collider.offset.x, transform.translation.y + collider.offset.y);
    }
}