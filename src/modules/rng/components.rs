use rand::rngs::StdRng;
use bevy::ecs::prelude::Resource;
use rand::SeedableRng;

#[derive(Resource)]
pub struct RngResource {
    pub generator: StdRng
}

impl Default for RngResource {
    fn default() -> Self {
        RngResource {generator: StdRng::from_entropy()}
    }
}

