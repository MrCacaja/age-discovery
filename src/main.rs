mod modules;

use bevy::app::{App, PluginGroup};
use bevy::DefaultPlugins;
use bevy::prelude::{Commands, ImagePlugin, IntoSystemDescriptor, Msaa};
use bevy::time::Timer;
use bevy::utils::default;
use bevy_ecs_ldtk::{LdtkPlugin, LevelSelection};
use bevy_ecs_ldtk::app::RegisterLdtkObjects;
use bevy_kira_audio::AudioPlugin;
use bevy_pixel_camera::PixelCameraPlugin;
use crate::modules::camera::systems::camera_follow;
use crate::modules::init::systems::{DebugPlugin, setup_game};
use crate::modules::input::systems::read_input;
use crate::modules::living::components::PlayerBundle;
use crate::modules::physics::sprite_change::systems::{overlap_sprite_zones, sync_self_physical_multiple_movement, update_movement_sided_sprite, update_sided_sprite};
use crate::modules::physics::systems::{collider_direction_react, direction_react, update_movement_state_by_direction, update_sideds_by_direction};
use crate::modules::prop::components::{RockBundle, TreeStumpBundle};
use crate::modules::rng::components::RngResource;
use crate::modules::sound::components::SoundEvent;
use crate::modules::sound::system::on_sound_emit;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(PixelCameraPlugin)
        .add_plugin(LdtkPlugin)
        .add_plugin(AudioPlugin)
        .add_plugin(DebugPlugin)
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_event::<SoundEvent>()
        .insert_resource(Msaa {samples: 1})
        .insert_resource(RngResource {..default()})
        .insert_resource(LevelSelection::Index(0))
        .register_ldtk_entity::<PlayerBundle>("Player")
        .register_ldtk_entity::<TreeStumpBundle>("Tree_Stump")
        .register_ldtk_entity::<RockBundle>("Rock")
        .add_startup_system(setup_game)
        .add_system(read_input)
        .add_system(on_sound_emit)
        .add_system(sync_self_physical_multiple_movement)
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
