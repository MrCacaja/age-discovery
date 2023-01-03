pub enum SoundType { RANDOM, FIXED }

pub struct SoundEvent {
    pub path: String,
    pub sound_type: SoundType,
    pub file: String,
}