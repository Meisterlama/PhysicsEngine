use std::collections::HashMap;
use std::time::Instant;

use bevy::prelude::*;

use crate::collision_plugin::collision_response;
use crate::collision_plugin::collision_response::ImpulseResult;
use crate::collision_plugin::config::CollisionConfig;
use crate::collision_plugin::data_structs::{CollisionResponseData, NarrowPhaseData};
use crate::collision_plugin::polygon_component::PolygonComponent;
use crate::collision_plugin::rigidbody::RigidBody2d;
use crate::transform2d::Transform2d;

pub(crate) fn collision_response(
    narrow_phase_data: ResMut<NarrowPhaseData>,
    mut collision_response_data: ResMut<CollisionResponseData>,
    mut query: Query<(&mut Transform2d, &mut RigidBody2d), With<PolygonComponent>>,
    config: Res<CollisionConfig>,
)
{
    let _span = info_span!("collision_response", name = "System").entered();
    let start = Instant::now();

    apply_response_st(&narrow_phase_data, &mut query, &config);
    // apply_response_mt(narrow_phase_data, &mut query, &config);    // FAILED ATTEMPT AT MULTITHREADING


    collision_response_data.time += Instant::now() - start;
}

pub(crate) fn apply_response_mt(narrow_phase_data: ResMut<NarrowPhaseData>, mut query: &mut Query<(&mut Transform2d, &mut RigidBody2d), With<PolygonComponent>>, config: &Res<CollisionConfig>) {
    let _span = info_span!("collision_response", name = "apply_multi_thread").entered();

    for _ in 0..64 {
        let impulses = narrow_phase_data.collision_infos.iter()
            .filter_map(|collision_info| {
                if let Some((imp_a, imp_b)) = collision_response::corrections::compute_collision_impulse(collision_info, &query, &config) {
                    return Some([imp_a, imp_b]);
                }
                return None;
            })
            .flatten()
            .collect::<Vec::<_>>();

        let mut impulse_set: HashMap<Entity, ImpulseResult> = HashMap::new();

        impulses.iter().for_each(|impulse| {
            let entity = impulse.entity.unwrap();
            if !impulse_set.contains_key(&entity) {
                let mut impulse = ImpulseResult::default();
                impulse.entity = Some(entity);
                impulse_set.insert(entity, impulse);
            }

            let ref_impulse = impulse_set.get_mut(&entity).unwrap();
            ref_impulse.linear_impulse += impulse.linear_impulse;
            ref_impulse.linear_impulse += impulse.angular_impulse;
        });

        impulse_set.iter().for_each(|(e, imp)| {
            let (_, mut rb) = query.get_mut(*e).unwrap();
            if !rb.is_kinematic {
                let maxlin = Vec2::new(10f32, 10f32);
                let maxang = 10f32;
                rb.linear_speed += imp.linear_impulse.clamp(-maxlin, maxlin);
                rb.angular_speed += imp.angular_impulse.clamp(-maxang, maxang);
            }
        });
    }

    narrow_phase_data.collision_infos.iter().for_each(|collision_info| {
        collision_response::corrections::apply_position_correction(collision_info, &mut query, &config);
    });
}

pub(crate) fn apply_response_st(narrow_phase_data: &ResMut<NarrowPhaseData>, mut query: &mut Query<(&mut Transform2d, &mut RigidBody2d), With<PolygonComponent>>, config: &Res<CollisionConfig>) {
    let _span = info_span!("collision_response", name = "apply_single_thread").entered();

    for _ in 0..16 {
        for collision_info in &narrow_phase_data.collision_infos {
            if let Some((imp_a, imp_b)) = collision_response::corrections::compute_collision_impulse(collision_info, &query, &config) {
                let (_, mut rb) = query.get_mut(imp_a.entity.unwrap()).unwrap();
                if !rb.is_kinematic {
                    rb.linear_speed += imp_a.linear_impulse;
                    rb.angular_speed += imp_a.angular_impulse;
                }
                let (_, mut rb) = query.get_mut(imp_b.entity.unwrap()).unwrap();
                if !rb.is_kinematic {
                    rb.linear_speed += imp_b.linear_impulse;
                    rb.angular_speed += imp_b.angular_impulse;
                }
            }
        }
    }

    for collision_info in &narrow_phase_data.collision_infos {
        collision_response::corrections::apply_friction(collision_info, &mut query, &config);
        collision_response::corrections::apply_position_correction(collision_info, &mut query, &config);
    }
}