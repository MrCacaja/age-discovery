use bevy::ecs::component::Component;
use bevy_inspector_egui::Inspectable;

pub mod living;
pub mod physics;

#[derive(Default, Component, Inspectable)]
pub struct Name(pub String);
