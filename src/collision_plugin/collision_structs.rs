use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Default, Clone, Copy, Inspectable)]
pub struct CollisionPair {
    pub entity_a: Option<Entity>,
    pub pos_a: Vec2,
    pub entity_b: Option<Entity>,
    pub pos_b: Vec2,
}

#[derive(Default, Inspectable)]
pub struct CollisionInfo {
    pub pair: CollisionPair,
    pub location: Vec2,
    pub normal: Vec2,
    pub distance: f32,
}