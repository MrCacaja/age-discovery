use bevy::math::Vec3;
use bevy::ecs::component::Component;
use bevy::prelude::{Query, Transform};
use bevy::ecs::bundle::Bundle;

#[derive(Default, Component)]
pub struct Physical {
    pub direction: Vec3,
    pub position: Vec3,
}

#[derive(Default, Component)]
pub struct SelfPhysical {
    pub self_direction: Vec3,
}

#[derive(Bundle, Default)]
pub struct SelfPhysicalBundle {
    physical: Physical,
    self_physical: SelfPhysical,
}

pub fn update_sprites(mut transforms: Query<(&mut Transform, &Physical)>) {
    for (mut transform, physical) in transforms.iter_mut() {
        transform.translation = physical.position;
    }
}

pub fn direction_react(mut entities: Query<(&mut Physical, Option<&SelfPhysical>)>) {
    for (mut physical, self_physical) in entities.iter_mut() {
        if let Some(self_physical) = self_physical {
            physical.direction += self_physical.self_direction;
        }

        let direction = physical.direction;
        physical.position += direction;
        
        if physical.direction.x > 0. {
            physical.direction.x -= 1.;
            if physical.direction.x < 0. {physical.direction.x = 0.};
        } else if physical.direction.x < 0. {
            physical.direction.x += 1.;
            if physical.direction.x > 0. {physical.direction.x = 0.};
        }

        if physical.direction.y > 0. {
            physical.direction.y -= 1.;
            if physical.direction.y < 0. {physical.direction.y = 0.};
        } else if physical.direction.y < 0. {
            physical.direction.y += 1.;
            if physical.direction.y > 0. {physical.direction.y = 0.};
        }
    }
}