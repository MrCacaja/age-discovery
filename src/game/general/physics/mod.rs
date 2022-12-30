use bevy::math::{Vec2, Vec3};
use bevy::ecs::component::Component;
use bevy::prelude::{Mut, Query, Res, TextureAtlasSprite, Transform};
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
pub struct Collider {
    pub size: Vec2,
    pub offset: Vec2
}

impl Default for Collider {
    fn default() -> Self {
        Self {
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
    time: Res<Time>, mut entities: Query<(&mut Physical, Option<&SelfPhysical>, &mut Transform)>,
) {
    for (mut physical, self_physical, mut transform) in entities.iter_mut() {
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

pub fn collider_direction_react(mut colliders: Query<(Option<&mut Physical>, Option<&mut SelfPhysical>, &mut Transform, &Collider)>) {
    let mut combinations = colliders.iter_combinations_mut();
    while let Some([(physical, self_physical, transform, collider), (target_physical, target_self_physical, target_transform, target_collider)]) = combinations.fetch_next() {
        if let Some(physical) = physical {
            if let Some(self_physical) = self_physical {
                collide_self(physical, self_physical, collider, transform, target_transform, target_collider);
            } else {
                collide(physical, collider, transform, target_transform, target_collider);
            }
        } else if let Some(target_physical) = target_physical {
            if let Some(target_self_physical) = target_self_physical {
                collide_self(target_physical, target_self_physical, target_collider, target_transform, transform, collider);
            } else {
                collide(target_physical, target_collider, target_transform, transform, collider);
            }
        }
    }

    fn collide_self(mut physical: Mut<'_, Physical, >, mut self_physical: Mut<'_, SelfPhysical, >,
               collider: &Collider, transform: Mut<'_, Transform, >,
               target_transform: Mut<'_, Transform, >, target_collider: &Collider) {
        let mut direction = physical.direction;
        direction += self_physical.direction;
        let collider_pos = Vec2 {x: transform.translation.x + collider.offset.x, y: transform.translation.y + collider.offset.y};
        let future_collider_pos_x = Vec2::new(direction.x + collider_pos.x, collider_pos.y);
        if check_future(future_collider_pos_x, collider, &target_transform, target_collider) {
            physical.direction.x = 0.;
            self_physical.direction.x = 0.;
        }
        let future_collider_pos_y = Vec2::new(collider_pos.x, direction.y + collider_pos.y);
        if check_future(future_collider_pos_y, collider, &target_transform, target_collider) {
            physical.direction.y = 0.;
            self_physical.direction.y = 0.;
        }
    }

    fn collide(mut physical: Mut<'_, Physical, >, collider: &Collider, transform: Mut<'_, Transform, >,
               target_transform: Mut<'_, Transform, >, target_collider: &Collider) {
        let collider_pos = Vec2 {x: transform.translation.x + collider.offset.x, y: transform.translation.y + collider.offset.y};
        let future_collider_pos_x = Vec2::new(physical.direction.x + collider_pos.x, collider_pos.y);
        if check_future(future_collider_pos_x, collider, &target_transform, target_collider) {
            physical.direction.x = 0.;
        }
        let future_collider_pos_y = Vec2::new(collider_pos.x, physical.direction.y + collider_pos.y);
        if check_future(future_collider_pos_y, collider, &target_transform, target_collider) {
            physical.direction.y = 0.;
        }
    }

    fn check_future(future_collider_pos: Vec2, collider: &Collider,
                    target_transform: &Mut<'_, Transform, >, target_collider: &Collider) -> bool {
        if future_collider_pos.x != 0. || future_collider_pos.y != 0. {
            let target_collider_pos = Vec2 {x: target_transform.translation.x + target_collider.offset.x, y: target_transform.translation.y + target_collider.offset.y};
            let target_collider_len = Vec2::new(
                target_collider_pos.x + target_collider.size.x, target_collider_pos.y + target_collider.size.y
            );
            let collider_len = future_collider_pos + collider.size;
            return !(
                collider_len.y          <   target_collider_pos.y   ||
                future_collider_pos.y   >   target_collider_len.y   ||
                collider_len.x          <   target_collider_pos.x   ||
                future_collider_pos.x   >   target_collider_len.x
            );
        }
        return false;
    }
}