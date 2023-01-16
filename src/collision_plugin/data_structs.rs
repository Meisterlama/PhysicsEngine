use std::collections::HashSet;
use std::time::Duration;

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Clone, Copy, Inspectable, Debug)]
pub struct CollisionPair {
    pub entity_a: Entity,
    pub entity_b: Entity,
}

#[derive(Default, Inspectable, Clone)]
pub struct CollisionInfo {
    pub collision_pair: Option<CollisionPair>,
    pub location: Vec<Vec2>,
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


#[derive(Default, Resource)]
pub struct BroadPhaseData {
    pub collision_pairs: Vec<CollisionPair>,
    pub sorted_entities: Vec<Entity>,
    pub time: Duration,
}

#[derive(Default, Resource)]
pub struct NarrowPhaseData {
    pub collision_infos: Vec<CollisionInfo>,
    pub collided_entities: HashSet<Entity>,
    pub time: Duration,
}

#[derive(Default, Resource)]
pub struct CollisionResponseData {
    pub time: Duration,
}