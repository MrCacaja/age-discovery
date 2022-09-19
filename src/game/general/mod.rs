use bevy::ecs::component::Component;

pub mod living;
pub mod physics;

#[derive(Default, Component)]
pub struct Name(pub String);
