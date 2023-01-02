use bevy::math::Vec2;
use bevy::prelude::{Mut, Query, Res, ResMut, TextureAtlasSprite, Time, Transform, Without};
use crate::modules::physics::sprite_change::consts::{GENERAL_BOTTOM, GENERAL_SIDE, GENERAL_TOP, MOB_BOTTOM_IDLE_END, MOB_BOTTOM_IDLE_START, MOB_BOTTOM_WALK_END, MOB_BOTTOM_WALK_START, MOB_SIDE_IDLE_END, MOB_SIDE_IDLE_START, MOB_SIDE_WALK_END, MOB_SIDE_WALK_START, MOB_TOP_IDLE_END, MOB_TOP_IDLE_START, MOB_TOP_WALK_END, MOB_TOP_WALK_START};
use crate::modules::physics::sprite_change::components::{MovementSpriteTimer, MovementState, MultipleMovementState, MultipleSided, Side, SpriteZone};

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

pub fn overlap_sprite_zones(mut sprite_zones: Query<(&SpriteZone, &mut Transform)>) {
    let mut combinations = sprite_zones.iter_combinations_mut();
    while let Some([(a_sprite_zone, mut a_transform), (b_sprite_zone, mut b_transform)]) = combinations.fetch_next() {
        let a_sprite_zone_pos = Vec2 {
            x: a_transform.translation.x + a_sprite_zone.0.offset.x,
            y: a_transform.translation.y + a_sprite_zone.0.offset.y
        };
        let a_sprite_zone_len = Vec2::new(
            a_sprite_zone_pos.x + a_sprite_zone.0.size.x, a_sprite_zone_pos.y + a_sprite_zone.0.size.y
        );
        let b_sprite_zone_pos = Vec2 {
            x: b_transform.translation.x + b_sprite_zone.0.offset.x,
            y: b_transform.translation.y + b_sprite_zone.0.offset.y
        };
        let b_sprite_zone_len = Vec2::new(
            b_sprite_zone_pos.x + b_sprite_zone.0.size.x, b_sprite_zone_pos.y + b_sprite_zone.0.size.y
        );
        let collided = !(
            a_sprite_zone_len.y   <   b_sprite_zone_pos.y   ||
                a_sprite_zone_pos.y   >   b_sprite_zone_len.y   ||
                a_sprite_zone_len.x   <   b_sprite_zone_pos.x   ||
                a_sprite_zone_pos.x   >   b_sprite_zone_len.x
        );
        if collided {
            if a_sprite_zone_pos.y < b_sprite_zone_pos.y {
                a_transform.translation.z = b_transform.translation.z + 0.1;
            } else {
                b_transform.translation.z = a_transform.translation.z + 0.1;
            }
        }
    }
}