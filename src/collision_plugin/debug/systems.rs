use std::mem;

use bevy::prelude::*;

use crate::collision_plugin::aabb::AABB;
use crate::collision_plugin::config::CollisionConfig;
use crate::collision_plugin::data_structs::{BroadPhaseData, CollisionResponseData, NarrowPhaseData};
use crate::collision_plugin::polygon_component::PolygonComponent;
use crate::collision_plugin::rendering::{LineBatch, LineBatches};
use crate::collision_plugin::rigidbody::RigidBody2d;
use crate::transform2d::Transform2d;

pub(crate) fn update_debug_info(
    query: Query<(), (With<PolygonComponent>, With<Transform2d>, With<AABB>)>,
    mut config: ResMut<CollisionConfig>,
    time: Res<Time>,
    broad_phase_data: ResMut<BroadPhaseData>,
    narrow_phase_data: ResMut<NarrowPhaseData>,
    collision_response_data: ResMut<CollisionResponseData>,
)
{
    config.statistics.entity_count = query.iter().size_hint().1.unwrap();
    config.statistics.broad_time = broad_phase_data.time.as_secs_f32() * 1000f32;
    config.statistics.narrow_time = narrow_phase_data.time.as_secs_f32() * 1000f32;
    config.statistics.collision_response_time = collision_response_data.time.as_secs_f32() * 1000f32;

    config.statistics.total_physics_time = config.statistics.broad_time + config.statistics.narrow_time + config.statistics.collision_response_time;
    config.statistics.total_frame_time = time.delta_seconds() * 1000f32;

    config.statistics.collision_pairs_count = broad_phase_data.collision_pairs.len();
    config.statistics.awake_entities_count = broad_phase_data.sorted_entities.len();
}

pub(crate) fn refresh_polygon_lines(
    mut line_batches: ResMut<LineBatches>,
    polygon_query: Query<(&PolygonComponent, &Transform2d)>,
)
{
    let mut batches = Vec::<LineBatch>::new();


    let mut non_colliding_batch = LineBatch::new(Color::GREEN);
    let mut colliding_batch = LineBatch::new(Color::RED);

    for (p, t) in &polygon_query {
        let mut points = p.get_transformed_points(t);

        points.push(points[0].clone());

        let points = points.iter().map(|p| p.extend(0f32)).collect::<Vec<_>>();

        if !p.collided {
            if !non_colliding_batch.try_push_vertices(&points)
            {
                batches.push(non_colliding_batch);
                non_colliding_batch = LineBatch::new(Color::GREEN);
                non_colliding_batch.try_push_vertices(&points);
            }
        } else {
            if !colliding_batch.try_push_vertices(&points)
            {
                batches.push(colliding_batch);
                colliding_batch = LineBatch::new(Color::RED);
                colliding_batch.try_push_vertices(&points);
            }
        }
    }

    if !colliding_batch.is_empty() {
        batches.push(colliding_batch);
    }
    if !non_colliding_batch.is_empty() {
        batches.push(non_colliding_batch);
    }

    line_batches.batches.append(&mut batches);
}

pub(crate) fn draw_debug(
    mut query: Query<(Entity, &mut PolygonComponent, &Transform2d, &RigidBody2d, &AABB)>,
    narrow_phase_data: ResMut<NarrowPhaseData>,
    broad_phase_data: ResMut<BroadPhaseData>,
    mut line_batches: ResMut<LineBatches>,
    config: Res<CollisionConfig>,
)
{
    let mut batches = Vec::<LineBatch>::new();

    if config.debug_drawing.draw_debug_aabb {
        let mut aabb_batch = LineBatch::new(Color::GRAY);

        query.iter_mut().for_each(|(_e, mut p, t, rb, a)| {
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
        query.iter_mut().for_each(|(_e, mut p, t, rb, a)| { p.collided = false; });
    }

    if config.debug_drawing.draw_debug_rigidbody {
        let mut velocity_batch = LineBatch::new(Color::AZURE);
        let mut acceleration_batch = LineBatch::new(Color::CRIMSON);

        for (e, p, t, rb, a) in query.iter() {
            let points = vec!(t.translation.extend(0f32), t.translation.extend(0f32) + rb.linear_speed.extend(0f32));
            if !velocity_batch.try_push_vertices(&points) {
                batches.push(velocity_batch);
                velocity_batch = LineBatch::new(Color::AZURE);
                velocity_batch.try_push_vertices(&points);
            }

            let points = vec!(t.translation.extend(0f32), t.translation.extend(0f32) + rb.linear_acceleration.extend(0f32));
            if !acceleration_batch.try_push_vertices(&points) {
                batches.push(acceleration_batch);
                acceleration_batch = LineBatch::new(Color::CRIMSON);
                acceleration_batch.try_push_vertices(&points);
            }
        }

        if !velocity_batch.is_empty() {
            batches.push(velocity_batch);
        }
        if !acceleration_batch.is_empty() {
            batches.push(acceleration_batch)
        }
    }


    if config.debug_drawing.draw_debug_broad_phase
    {
        let mut broad_phase_batch = LineBatch::new(Color::YELLOW);

        for pair in &broad_phase_data.collision_pairs {
            let (_, p1, t1, rb, a1) = query.get(pair.entity_a).unwrap();
            let (_, p2, t2, rb, a2) = query.get(pair.entity_b).unwrap();
            let points = vec!(
                t1.translation.extend(0f32),
                t2.translation.extend(0f32),
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

    if config.debug_drawing.draw_debug_narrow_phase {
        let mut narrow_phase_batch = LineBatch::new(Color::CYAN);

        for collision_info in &narrow_phase_data.collision_infos {
            if let Some(pair) = collision_info.collision_pair
            {
                {
                    let (_, mut p1, t1, rb, a1) = query.get_mut(pair.entity_a).unwrap();
                    p1.collided = true;
                }

                {
                    let (_, mut p2, t2, rb, a2) = query.get_mut(pair.entity_b).unwrap();
                    p2.collided = true;
                }

                for location_info in &collision_info.location {
                    let points = vec!(
                        location_info.extend(0f32),
                        location_info.extend(0f32)
                            + collision_info.normal.extend(0f32) * collision_info.distance,
                    );
                    if !narrow_phase_batch.try_push_vertices(&points)
                    {
                        batches.push(narrow_phase_batch);
                        narrow_phase_batch = LineBatch::new(Color::CYAN);
                        narrow_phase_batch.try_push_vertices(&points);
                    }
                }
            }
        }
        if !narrow_phase_batch.is_empty() {
            batches.push(narrow_phase_batch);
        }
    }

    line_batches.batches.append(&mut batches);
}