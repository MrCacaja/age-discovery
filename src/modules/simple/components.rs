use bevy::ecs::component::Component;
use bevy_inspector_egui::Inspectable;


#[derive(Default, Component, Inspectable)]
pub struct Name(pub String);
