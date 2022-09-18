use bevy::ecs::component::Component;

pub mod living;

#[derive(Default, Component)]
pub struct Name(pub String);
