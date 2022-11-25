use std::time::Instant;
use bevy::prelude::*;
use rayon::prelude::*;


use crate::aabb::{AABB, check_collision};
use crate::collision_plugin::SystemData;
use crate::collision_plugin::collision_structs::CollisionPair;
use crate::collision_plugin::config::{BroadPhaseType, CollisionConfig};
use crate::polygon_component::PolygonComponent;
use crate::transform2d::Transform2d;

type BroadPhaseQuery<'w, 's> = Query<'w, 's, (Entity, &'static PolygonComponent, &'static Transform2d, &'static AABB)>;

pub fn broad_phase(query: BroadPhaseQuery,
                   mut system_data: ResMut<SystemData>,
                   config: ResMut<CollisionConfig>)
{
    let start = Instant::now();

    if let Some(collision_pairs) = &mut system_data.broad_phase_collision_pairs
    {
        collision_pairs.clear();
    }

    system_data.broad_phase_collision_pairs = match config.broad_phase_type {
        BroadPhaseType::Disabled => None,
        BroadPhaseType::Rough => Some(broad_phase_rough(&query)),
        BroadPhaseType::SAP => Some(broad_phase_sap(&query)),
    };

    system_data.broad_time = Instant::now() - start;
}

//Naively add every pairs to check
fn broad_phase_rough(query: &BroadPhaseQuery) -> Vec<CollisionPair>
{
    let collision_pairs = query.iter_combinations().collect::<Vec<_>>().par_iter().filter_map(|[(e1, _p1, t1, _a1), (e2, _p2, t2, _a2)]| {
        return Some(CollisionPair {
            entity_a: Some(*e1),
            pos_a: t1.position,
            entity_b: Some(*e2),
            pos_b: t2.position,
        });
    }).collect();
    return collision_pairs;
}

//Use Sweep & Prune to create a vector of CollisionPair
fn broad_phase_sap(query: &BroadPhaseQuery) -> Vec<CollisionPair>
{
    let mut query_iter_sorted = query.iter().collect::<Vec<_>>();

    query_iter_sorted.sort_by(|(_e1, _p1, t1, a1), (_e2, _p2, t2, a2)| {
        return (t1.translate(&a1.min).x).partial_cmp(&(t2.translate(&a2.min).x)).unwrap();
    });

    let query_iter_sorted = query_iter_sorted;

    let collision_pairs: Vec<CollisionPair> = query_iter_sorted.par_iter().enumerate().filter_map(|(i, (e1, _p1, t1, a1))|
        {
            let mut tmp_collision = Vec::<CollisionPair>::new();
            for (e2, _p2, t2, a2) in query_iter_sorted.iter().skip(i+1)
            {
                if t2.translate(&a2.min).x > t1.translate(&a1.max).x { break; };

                if check_collision(&a1, &t1, &a2, &t2)
                {
                    tmp_collision.push(CollisionPair {
                        entity_a: Some(*e1),
                        pos_a: t1.position,
                        entity_b: Some(*e2),
                        pos_b: t2.position,
                    });
                }
            }
            return Some(tmp_collision);
        }).flatten().collect();
    return collision_pairs;
}