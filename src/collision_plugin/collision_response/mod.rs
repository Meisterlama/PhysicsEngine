use bevy::prelude::*;

pub mod corrections;
pub mod systems;

#[derive(Default)]
pub struct ImpulseResult {
    pub entity: Option<Entity>,
    pub linear_impulse: Vec2,
    pub angular_impulse: f32,
}