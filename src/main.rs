mod game;

use bevy::app::{App};
use bevy::DefaultPlugins;
use bevy::prelude::{Camera2dBundle, Commands, OrthographicProjection, ParallelSystemDescriptorCoercion};
use bevy::render::camera::ScalingMode;
use bevy::render::texture::ImageSettings;
use bevy::time::Timer;
use bevy::utils::default;
use bevy_ecs_ldtk::{LdtkPlugin, LevelSelection};
use bevy_ecs_ldtk::app::RegisterLdtkObjects;
use crate::game::general::Name;
use crate::game::{camera_follow, DebugPlugin, read_input, setup_game};
use crate::game::general::living::player::PlayerBundle;
use crate::game::general::physics::{Collider, collider_direction_react, direction_react, MovementSpriteTimer, overlap_sprite_zones, Physical, update_movement_sided_sprite, update_movement_state_by_direction, update_sided_sprite, update_sideds_by_direction};
use crate::game::general::props::{RockBundle, TreeStumpBundle};

// const GENERAL_SPRITE_SHEET_COLS: usize = 3;
// const GENERAL_SPRITE_SHEET_ROWS: usize = 1;
const GENERAL_TOP: usize = 2;
const GENERAL_SIDE: usize = 1;
const GENERAL_BOTTOM: usize = 0;

const MOB_SPRITE_SHEET_COLS: usize = 5;
//const MOB_SPRITE_SHEET_ROWS: usize = 3;

const MOB_BOTTOM_IDLE_START: usize = 0;
const MOB_BOTTOM_IDLE_END: usize = MOB_BOTTOM_IDLE_START + 2;

const MOB_BOTTOM_WALK_START: usize = MOB_BOTTOM_IDLE_END + 1;
const MOB_BOTTOM_WALK_END: usize = MOB_BOTTOM_WALK_START;

const MOB_TOP_IDLE_START: usize = MOB_SPRITE_SHEET_COLS * 2;
const MOB_TOP_IDLE_END: usize = MOB_TOP_IDLE_START + 2;

const MOB_TOP_WALK_START: usize = MOB_TOP_IDLE_END + 1;
const MOB_TOP_WALK_END: usize = MOB_TOP_WALK_START;

const MOB_SIDE_IDLE_START: usize = MOB_SPRITE_SHEET_COLS;
const MOB_SIDE_IDLE_END: usize = MOB_SIDE_IDLE_START + 2;

const MOB_SIDE_WALK_START: usize = MOB_SIDE_IDLE_END + 1;
const MOB_SIDE_WALK_END: usize = MOB_SIDE_WALK_START + 1;

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .add_plugin(DebugPlugin)
        .insert_resource(MovementSpriteTimer{timer: Timer::from_seconds(0.2, true)})
        .insert_resource(LevelSelection::Index(0))
        .register_ldtk_entity::<PlayerBundle>("Player")
        .register_ldtk_entity::<TreeStumpBundle>("Tree_Stump")
        .register_ldtk_entity::<RockBundle>("Rock")
        .add_startup_system(setup_game)
        .add_system(read_input)
        .add_system(collider_direction_react.after(read_input))
        .add_system(direction_react.after(collider_direction_react))
        .add_system(overlap_sprite_zones.after(direction_react))
        .add_system(update_movement_state_by_direction.after(collider_direction_react))
        .add_system(update_movement_sided_sprite.after(update_movement_state_by_direction))
        .add_system(update_sideds_by_direction.after(direction_react))
        .add_system(update_sided_sprite.after(update_sideds_by_direction))
        .add_system(camera_follow.after(update_sided_sprite))
        .run();
}
