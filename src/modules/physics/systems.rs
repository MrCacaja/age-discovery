use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Changed, Mut, Or, Query, Res, Time, Transform};
use crate::modules::physics::components::{Collider, Physical, SelfPhysical};
use crate::modules::physics::sprite_change::components::{MovementState, MultipleMovementState, MultipleSided, Side};


pub fn update_movement_state_by_direction(
    mut entities: Query<(&mut MultipleMovementState, &Physical, Option<&SelfPhysical>), Or<(Changed<Physical>, Changed<SelfPhysical>)>>
) {
    for (mut movement_state, physical, self_physical) in entities.iter_mut() {
        let being_pushed = physical.direction.x != 0. || physical.direction.y != 0.;
        if being_pushed {
            movement_state.state = MovementState::DRAG;
        } else {
            movement_state.state = MovementState::IDLE;
        }
        if let Some(self_physical) = self_physical {
            let moving = self_physical.direction.x != 0. || self_physical.direction.y != 0.;
            if (physical.acceleration < self_physical.speed || !being_pushed) && moving {
                movement_state.state = MovementState::WALK;
            }
        }
    }
}

pub fn update_sideds_by_direction(
    mut multiple_sideds: Query<(&Physical, &mut MultipleSided, Option<&SelfPhysical>), Or<(Changed<Physical>, Changed<SelfPhysical>)>>
) {
    for (physical, mut multiple_sided, self_physical) in multiple_sideds.iter_mut() {
        let mut direction = physical.direction;

        if let Some(self_physical) = self_physical {
            if physical.acceleration < self_physical.speed {
                direction = self_physical.direction;
            }
        }

        direction.x =  f32::trunc(direction.x  * 100.0) / 100.0;
        direction.y =  f32::trunc(direction.y  * 100.0) / 100.0;

        if direction.x > 0. {
            multiple_sided.side = Side::RIGHT;
        }
        else if direction.x < 0. {
            multiple_sided.side = Side::LEFT;
        }
        else if direction.y < 0. {
            multiple_sided.side = Side::BOTTOM;
        }
        else if direction.y > 0. {
            multiple_sided.side = Side::TOP;
        }
    }
}

pub fn direction_react(
    time: Res<Time>, mut entities: Query<(&mut Physical, Option<&SelfPhysical>, &mut Transform)>,
) {
    for (mut physical, self_physical, mut transform) in entities.iter_mut() {
        if let Some(self_physical) = self_physical {
            if self_physical.speed > physical.acceleration && self_physical.direction != Vec3::ZERO {
                transform.translation += self_physical.direction * time.delta_seconds() * self_physical.speed;
            }
        }

        if physical.direction != Vec3::ZERO {
            physical.direction = physical.direction.normalize_or_zero();
            transform.translation += physical.direction * time.delta_seconds() * physical.acceleration;
            physical.acceleration = (physical.acceleration - physical.weight).clamp(0., f32::MAX);
        }
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
        let collider_pos = Vec2 {x: transform.translation.x + collider.0.offset.x, y: transform.translation.y + collider.0.offset.y};
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
        let collider_pos = Vec2 {x: transform.translation.x + collider.0.offset.x, y: transform.translation.y + collider.0.offset.y};
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
            let target_collider_pos = Vec2 {
                x: target_transform.translation.x + target_collider.0.offset.x,
                y: target_transform.translation.y + target_collider.0.offset.y
            };
            let target_collider_len = Vec2::new(
                target_collider_pos.x + target_collider.0.size.x, target_collider_pos.y + target_collider.0.size.y
            );
            let collider_len = future_collider_pos + collider.0.size;
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