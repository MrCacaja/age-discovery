use bevy::prelude::{Camera, Changed, Query, Transform, With, Without};
use crate::modules::camera::components::CameraTarget;

pub fn camera_follow(
    camera_targets: Query<&mut Transform, (With<CameraTarget>, Changed<Transform>)>,
    mut cameras: Query<&mut Transform, (With<Camera>, Without<CameraTarget>)>
) {
    match camera_targets.get_single() {
        Ok(target) => {
            let mut camera = cameras.get_single_mut().unwrap();
            camera.translation.x = target.translation.x;
            camera.translation.y = target.translation.y;
        }
        _ => {}
    };
}