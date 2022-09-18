mod game;

use bevy::app::{App};
use bevy::DefaultPlugins;
use bevy::prelude::{Camera2dBundle, Commands, OrthographicProjection};
use bevy::render::camera::ScalingMode;
use bevy::render::texture::ImageSettings;
use bevy::utils::default;
use crate::game::general::Name;
use crate::game::general::living::{Person};
use crate::game::{read_input, setup_game};

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_game)
        .add_system(read_input)
        .run();
}
