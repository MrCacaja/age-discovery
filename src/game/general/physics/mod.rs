use bevy::math::{Vec2, Vec3};
use bevy::ecs::component::Component;
use bevy::prelude::{Mut, Query, Res, ResMut, TextureAtlasSprite, Transform, Without};
use bevy::ecs::bundle::Bundle;
use bevy::time::{Time, Timer};
use bevy_ecs_ldtk::{EntityInstance, LdtkEntity};
use bevy_inspector_egui::Inspectable;
use crate::{default, GENERAL_BOTTOM, GENERAL_SIDE, GENERAL_TOP, MOB_BOTTOM_IDLE_END, MOB_BOTTOM_IDLE_START, MOB_BOTTOM_WALK_END, MOB_BOTTOM_WALK_START, MOB_SIDE_IDLE_END, MOB_SIDE_IDLE_START, MOB_SIDE_WALK_END, MOB_SIDE_WALK_START, MOB_TOP_IDLE_END, MOB_TOP_IDLE_START, MOB_TOP_WALK_END, MOB_TOP_WALK_START};

#[derive(Inspectable, Debug)]
enum Side { BOTTOM, LEFT, RIGHT, TOP }

impl Default for Side {
    fn default() -> Self {
        Side::BOTTOM
    }
}

#[derive(Inspectable, Debug)]
enum MovementState { IDLE, WALK, DRAG }

impl Default for MovementState {
    fn default() -> Self {
        MovementState::IDLE
    }
}

#[derive(Default, Component, Inspectable)]
pub struct MultipleSided {
    side: Side
}

#[derive(Default, Component, Inspectable)]
pub struct MultipleMovementState {
    current_index: usize,
    state: MovementState,
    used_first: bool
}

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

pub fn update_movement_state_by_direction(mut entities: Query<(&mut MultipleMovementState, &Physical, Option<&SelfPhysical>)>) {
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

pub fn update_sideds_by_direction(mut multiple_sideds: Query<(&Physical, &mut MultipleSided, Option<&SelfPhysical>)>) {
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

pub fn update_movement_sided_sprite(
    time: Res<Time>, mut timer: ResMut<MovementSpriteTimer>,
    mut multiple_sideds: Query<(&MultipleSided, &mut TextureAtlasSprite, &mut MultipleMovementState)>
) {
    let mut should_increase = false;
    if timer.timer.tick(time.delta()).just_finished() {
        should_increase = true;
    }
    for (multiple_sided, atlas_sprite, multiple_movement_state) in multiple_sideds.iter_mut() {
        match multiple_movement_state.state {
            MovementState::IDLE => {
                match multiple_sided.side {
                    Side::TOP =>
                        update_idle_sprite(
                            multiple_movement_state, atlas_sprite, MOB_TOP_IDLE_START,
                            MOB_TOP_IDLE_END, should_increase, false
                        ),
                    Side::BOTTOM =>
                        update_idle_sprite(
                            multiple_movement_state, atlas_sprite, MOB_BOTTOM_IDLE_START,
                            MOB_BOTTOM_IDLE_END, should_increase, false
                        ),
                    Side::LEFT =>
                        update_idle_sprite(
                            multiple_movement_state, atlas_sprite, MOB_SIDE_IDLE_START,
                            MOB_SIDE_IDLE_END, should_increase, false
                        ),
                    Side::RIGHT =>
                        update_idle_sprite(
                            multiple_movement_state, atlas_sprite, MOB_SIDE_IDLE_START,
                            MOB_SIDE_IDLE_END, should_increase, true
                        )
                }
            }
            MovementState::WALK => {
                match multiple_sided.side {
                    Side::TOP =>
                        update_walk_sprite(
                            multiple_movement_state, atlas_sprite, MOB_TOP_WALK_START,
                            MOB_TOP_WALK_END, MOB_TOP_IDLE_START, should_increase, true, false
                        ),
                    Side::BOTTOM =>
                        update_walk_sprite(
                            multiple_movement_state, atlas_sprite, MOB_BOTTOM_WALK_START,
                            MOB_BOTTOM_WALK_END, MOB_BOTTOM_IDLE_START, should_increase, true, false
                        ),
                    Side::LEFT =>
                        update_walk_sprite(
                            multiple_movement_state, atlas_sprite, MOB_SIDE_WALK_START,
                            MOB_SIDE_WALK_END, MOB_SIDE_IDLE_START, should_increase, false, false
                        ),
                    Side::RIGHT =>
                        update_walk_sprite(
                            multiple_movement_state, atlas_sprite, MOB_SIDE_WALK_START,
                            MOB_SIDE_WALK_END, MOB_SIDE_IDLE_START, should_increase, false, true
                        )
                }
            }
            MovementState::DRAG => {} //TODO
        }
    }

    fn update_walk_sprite(
        mut multiple_movement_state: Mut<'_, MultipleMovementState>, mut atlas_sprite: Mut<'_, TextureAtlasSprite>,
        start_index: usize, end_index: usize, idle_index: usize, should_increase: bool, auto_flip_x: bool, flip_x: bool
    ) {
        if (multiple_movement_state.current_index < start_index || multiple_movement_state.current_index > end_index) &&
            multiple_movement_state.current_index != idle_index {
            multiple_movement_state.current_index = start_index;
            multiple_movement_state.used_first = true;
            if auto_flip_x {
                atlas_sprite.flip_x = !atlas_sprite.flip_x;
            }
        } else if should_increase {
            if multiple_movement_state.current_index == idle_index {
                if auto_flip_x {
                    multiple_movement_state.current_index = start_index;
                    atlas_sprite.flip_x = !atlas_sprite.flip_x;
                } else {
                    if multiple_movement_state.used_first {
                        multiple_movement_state.current_index = end_index;
                    } else {
                        multiple_movement_state.current_index = start_index;
                    }
                    multiple_movement_state.used_first = !multiple_movement_state.used_first;
                }
            } else {
                multiple_movement_state.current_index = idle_index;
            }
        }
        if !auto_flip_x {
            atlas_sprite.flip_x = flip_x;
        }
        atlas_sprite.index = multiple_movement_state.current_index;
    }

    fn update_idle_sprite(
        mut multiple_movement_state: Mut<'_, MultipleMovementState>, mut atlas_sprite: Mut<'_, TextureAtlasSprite>,
        start_index: usize, end_index: usize, should_increase: bool, flip_x: bool
    ) {
        if multiple_movement_state.current_index >= end_index ||
            multiple_movement_state.current_index < start_index {
            multiple_movement_state.current_index = start_index;
        } else if should_increase {
            multiple_movement_state.current_index += 1;
        }
        atlas_sprite.index = multiple_movement_state.current_index;
        atlas_sprite.flip_x = flip_x;
    }
}

pub fn update_sided_sprite(mut multiple_sideds: Query<(&MultipleSided, &mut TextureAtlasSprite), Without<MultipleMovementState>>) {
    for (multiple_sided, mut atlas_sprite) in multiple_sideds.iter_mut() {
        match multiple_sided.side {
            Side::TOP => {
                atlas_sprite.index = GENERAL_TOP;
                atlas_sprite.flip_x = false;
            }
            Side::BOTTOM => {
                atlas_sprite.index = GENERAL_BOTTOM;
                atlas_sprite.flip_x = false;
            }
            Side::LEFT => {
                atlas_sprite.index = GENERAL_SIDE;
                atlas_sprite.flip_x = false;
            }
            Side::RIGHT => {
                atlas_sprite.index = GENERAL_SIDE;
                atlas_sprite.flip_x = true;
            }
        }
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