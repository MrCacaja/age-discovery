use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Mut, Query, Res, ResMut, Transform};
use bevy::time::Time;
use rand::Rng;
use crate::modules::living::components::{Desire, Intention};
use crate::modules::physics::components::SelfPhysical;
use crate::modules::rng::components::RngResource;

pub fn update_mob_direction(mut mobs: Query<(&Desire, &mut SelfPhysical, &Transform)>) {
    for (desire, mut self_physical, transform) in mobs.iter_mut() {
        match desire.intention {
            Intention::WalkTo => {
                let direction = Vec2::new(desire.position.x - transform.translation.x, desire.position.y - transform.translation.y).normalize();
                self_physical.direction.x = direction.x;
                self_physical.direction.y = direction.y;
            },
            _ => {
                self_physical.direction = Vec3::ZERO;
            }
        }
    }
}

pub fn update_desire(time: Res<Time>, mut rng: ResMut<RngResource>, mut mobs: Query<(&mut Desire, &Transform)>) {
    for (mut desire, transform) in mobs.iter_mut() {
        match desire.intention {
            Intention::Idle => {
                change_desire(desire, &time, &mut rng, transform)
            },
            Intention::WalkTo => {
                if ((desire.position.x - 1.)..(desire.position.x + 1.)).contains(&transform.translation.x)
                    && ((desire.position.y - 1.)..(desire.position.y + 1.)).contains(&transform.translation.y) {
                    desire.intention = Intention::Idle;
                }
            },
            _ => {},
        }
    }

    fn change_desire(mut desire: Mut<Desire>, time: &Res<Time>, rng: &mut ResMut<RngResource>, transform: &Transform) {
        if desire.update_timer.tick(time.delta()).just_finished() {
            match rng.generator.gen_range(0..2) {
                0 => {
                    desire.intention = Intention::Idle;
                    desire.position = Vec2::ZERO;
                },
                1 => {
                    desire.intention = Intention::WalkTo;
                    desire.position.x = rng.generator.gen_range((transform.translation.x - 100.)..(transform.translation.x + 100.));
                    desire.position.y = rng.generator.gen_range((transform.translation.y - 100.)..(transform.translation.y + 100.));
                },
                _ => {}
            }
        }
    }
}