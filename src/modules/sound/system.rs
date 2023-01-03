use bevy::asset::AssetServer;
use bevy::prelude::{EventReader, Res, ResMut};
use bevy_kira_audio::{Audio, AudioControl};
use rand::Rng;
use crate::modules::rng::components::RngResource;
use crate::modules::sound::components::{SoundEvent, SoundType};

pub fn on_sound_emit(
    mut rng: ResMut<RngResource>, audio: Res<Audio>, asset_server: Res<AssetServer>,
    mut ev_sound: EventReader<SoundEvent>
) {
    for ev in ev_sound.iter() {
        let sound_path = "sounds/".to_owned() + &*ev.path + "/" +
            &*match ev.sound_type {
                SoundType::RANDOM => (rng.generator.gen::<u32>() % 4).to_string(),
                SoundType::FIXED => ev.file.to_string(),
            } +".wav";
        audio.play(asset_server.load(sound_path));
    }
}