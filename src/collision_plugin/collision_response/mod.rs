use std::time::{Duration, Instant};
use bevy::prelude::*;
use crate::collision_plugin::CollisionStage;
use crate::collision_plugin::narrow_phase::NarrowPhaseData;
use crate::collision_plugin::rigidbody::RigidBody2d;
use crate::polygon_component::PolygonComponent;
use crate::polygon_plugin::EntityToMove;
use crate::transform2d::Transform2d;

pub struct CollisionResponsePlugin;

impl Plugin for CollisionResponsePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CollisionResponseData>()
            .add_system_to_stage(CollisionStage::CollisionResponse, collision_response);
    }
}

fn collision_response(
    narrow_phase_data: ResMut<NarrowPhaseData>,
    mut collision_response_data: ResMut<CollisionResponseData>,
    mut query: Query<(&mut Transform2d, &RigidBody2d), (With<PolygonComponent>)>,
)
{
    let start = Instant::now();

    for collision_info in &narrow_phase_data.collision_infos {
        if let Some(collision_pair) = collision_info.collision_pair {
            let [(mut t1, rb1), (mut t2, rb2)] = query.get_many_mut([collision_pair.entity_a, collision_pair.entity_b]).unwrap();

            let inv_mass_rb1 = 1f32 / rb1.get_mass();
            let inv_mass_rb2 = 1f32 / rb2.get_mass();

            let j = (-1f32) / (inv_mass_rb1 + inv_mass_rb2);
            if !rb1.is_kinematic {
                t1.position += collision_info.normal * collision_info.distance * j * inv_mass_rb1;
            }

            if !rb2.is_kinematic {
                t2.position -= collision_info.normal * collision_info.distance * j * inv_mass_rb2;
            }
        }
    }
    collision_response_data.time = Instant::now() - start;
}

#[derive(Default, Resource)]
pub struct CollisionResponseData {
    pub time: Duration,
}