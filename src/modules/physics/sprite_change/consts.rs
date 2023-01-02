#![allow(dead_code)]

pub const GENERAL_SPRITE_SHEET_COLS: usize = 3;
pub const GENERAL_SPRITE_SHEET_ROWS: usize = 1;
pub const GENERAL_TOP: usize = 2;
pub const GENERAL_SIDE: usize = 1;
pub const GENERAL_BOTTOM: usize = 0;

pub const MOB_SPRITE_SHEET_COLS: usize = 5;
pub const MOB_SPRITE_SHEET_ROWS: usize = 3;

pub const MOB_BOTTOM_IDLE_START: usize = 0;
pub const MOB_BOTTOM_IDLE_END: usize = MOB_BOTTOM_IDLE_START + 2;

pub const MOB_BOTTOM_WALK_START: usize = MOB_BOTTOM_IDLE_END + 1;
pub const MOB_BOTTOM_WALK_END: usize = MOB_BOTTOM_WALK_START;

pub const MOB_TOP_IDLE_START: usize = MOB_SPRITE_SHEET_COLS * 2;
pub const MOB_TOP_IDLE_END: usize = MOB_TOP_IDLE_START + 2;

pub const MOB_TOP_WALK_START: usize = MOB_TOP_IDLE_END + 1;
pub const MOB_TOP_WALK_END: usize = MOB_TOP_WALK_START;

pub const MOB_SIDE_IDLE_START: usize = MOB_SPRITE_SHEET_COLS;
pub const MOB_SIDE_IDLE_END: usize = MOB_SIDE_IDLE_START + 2;

pub const MOB_SIDE_WALK_START: usize = MOB_SIDE_IDLE_END + 1;
pub const MOB_SIDE_WALK_END: usize = MOB_SIDE_WALK_START + 1;