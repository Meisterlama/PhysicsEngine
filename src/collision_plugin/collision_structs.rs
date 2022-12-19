use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Clone, Copy, Inspectable)]
pub struct CollisionPair {
    pub entity_a: Entity,
    pub entity_b: Entity,
}

#[derive(Default, Inspectable)]
pub struct CollisionInfo {
    pub collision_pair: Option<CollisionPair>,
    pub location: Vec2,
    pub normal: Vec2,
    pub distance: f32,
}

impl CollisionInfo {
    pub fn new(collision_pair: &CollisionPair) -> Self {
        Self {
            collision_pair: Some(collision_pair.clone()),
            ..Default::default()
        }
    }
}