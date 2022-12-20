use std::mem;
use bevy::prelude::*;

use crate::aabb::AABB;
use crate::collision_plugin::broad_phase::BroadPhaseData;
use crate::collision_plugin::config::CollisionConfig;
use crate::collision_plugin::narrow_phase::NarrowPhaseData;
use crate::polygon_component::PolygonComponent;
use crate::line_renderer::{LineBatch, LineBatches};
use crate::transform2d::Transform2d;

pub fn draw_debug(
    mut query: Query<(Entity, &mut PolygonComponent, &Transform2d, &AABB)>,
    narrow_phase_data: ResMut<NarrowPhaseData>,
    broad_phase_data: ResMut<BroadPhaseData>,
    mut line_batches: ResMut<LineBatches>,
    config: Res<CollisionConfig>,
)
{
    let mut batches = Vec::<LineBatch>::new();

    if config.draw_debug_aabb {
        let mut aabb_batch = LineBatch::new(Color::GRAY);

        query.iter_mut().for_each(|(_e, mut p, t, a)| {
            let points = a.get_points(t);
            if !aabb_batch.try_push_vertices(&points)
            {
                let mut tmp = LineBatch::new(Color::GRAY);
                mem::swap(&mut aabb_batch, &mut tmp);
                batches.push(tmp);
                aabb_batch.try_push_vertices(&points);
            }

            p.collided = false;
        });

        if !aabb_batch.is_empty() {
            batches.push(aabb_batch);
        }
    } else {
        query.iter_mut().for_each(|(_e, mut p, t, a)| { p.collided = false; });
    }


    if config.draw_debug_broad_phase
    {
        let mut broad_phase_batch = LineBatch::new(Color::YELLOW);

        for pair in &broad_phase_data.collision_pairs {
            let (_, p1, t1, a1) = query.get(pair.entity_a).unwrap();
            let (_, p2, t2, a2) = query.get(pair.entity_b).unwrap();
            let points = vec!(
                t1.position.extend(0f32),
                t2.position.extend(0f32),
            );
            if !broad_phase_batch.try_push_vertices(&points)
            {
                batches.push(broad_phase_batch);
                broad_phase_batch = LineBatch::new(Color::YELLOW);
                broad_phase_batch.try_push_vertices(&points);
            }
        }

        if !broad_phase_batch.is_empty() {
            batches.push(broad_phase_batch);
        }
    }

    if config.draw_debug_narrow_phase {
        let mut narrow_phase_batch = LineBatch::new(Color::CYAN);

        for collision_info in &narrow_phase_data.collision_infos {
            if let Some(pair) = collision_info.collision_pair
            {
                {
                    let (_, mut p1, t1, a1) = query.get_mut(pair.entity_a).unwrap();
                    p1.collided = true;
                }

                {
                    let (_, mut p2, t2, a2) = query.get_mut(pair.entity_b).unwrap();
                    p2.collided = true;
                }

                let points = vec!(
                    collision_info.location.extend(0f32),
                    collision_info.location.extend(0f32)
                        - collision_info.normal.extend(0f32) * collision_info.distance,
                );
                if !narrow_phase_batch.try_push_vertices(&points)
                {
                    batches.push(narrow_phase_batch);
                    narrow_phase_batch = LineBatch::new(Color::CYAN);
                    narrow_phase_batch.try_push_vertices(&points);
                }
            }
        }
        if !narrow_phase_batch.is_empty() {
            batches.push(narrow_phase_batch);
        }
    }
    line_batches.batches.append(&mut batches);
}