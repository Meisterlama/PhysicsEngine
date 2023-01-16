use std::time::Instant;

use bevy::prelude::*;
use rayon::prelude::*;

use crate::{
    collision_plugin::{
        aabb::{AABB, check_collision},
        data_structs::CollisionPair,
        PhysicsAwake,
        polygon_component::PolygonComponent,
    }
};
use crate::collision_plugin::config::{BroadPhaseType, CollisionConfig};
use crate::collision_plugin::data_structs::BroadPhaseData;
use crate::transform2d::Transform2d;

pub type BroadPhaseQueryAwake<'w, 's> = Query<
    'w, 's,
    (
        Entity,
        &'static PolygonComponent,
        &'static Transform2d,
        &'static AABB
    ),
    With<PhysicsAwake>
>;


pub(crate) fn broad_phase(
    query: BroadPhaseQueryAwake,
    mut broad_phase_data: ResMut<BroadPhaseData>,
    config: ResMut<CollisionConfig>)
{
    let _span = info_span!("broad_phase", name = "dispatching").entered();

    let start = Instant::now();
    broad_phase_data.collision_pairs.clear();

    if config.phase_config.multithread_broad_phase {
        match config.phase_config.broad_phase_type {
            BroadPhaseType::Disabled => {}
            BroadPhaseType::Rough => {
                broad_phase_data.collision_pairs = compute_collision_pairs_rough_mt(&query, &broad_phase_data.sorted_entities);
            }
            BroadPhaseType::SAP => {
                broad_phase_data.collision_pairs = compute_collision_pairs_sap_mt(&query, &broad_phase_data.sorted_entities);
            }
        }
    } else {
        match config.phase_config.broad_phase_type {
            BroadPhaseType::Disabled => {}
            BroadPhaseType::Rough => {
                broad_phase_data.collision_pairs = compute_collision_pairs_rough_st(&query, &broad_phase_data.sorted_entities);
            }
            BroadPhaseType::SAP => {
                broad_phase_data.collision_pairs = compute_collision_pairs_sap_st(&query, &broad_phase_data.sorted_entities);
            }
        }
    }

    broad_phase_data.collision_pairs.iter().for_each(|pair| assert_ne!(pair.entity_a, pair.entity_b));

    broad_phase_data.time += Instant::now() - start;
}


//Naively add every pairs to check
#[must_use]
pub(crate) fn compute_collision_pairs_rough_mt(
    query: &BroadPhaseQueryAwake,
    entities: &Vec<Entity>,
) -> Vec<CollisionPair>
{
    let _span = info_span!("broad_phase", name = "compute rough collisions").entered();

    let mut collision_pairs = vec!();
    collision_pairs.par_extend(query.iter_combinations().collect::<Vec<_>>().par_iter().filter_map(|[(e1, _p1, t1, _a1), (e2, _p2, t2, _a2)]| {
        return Some(CollisionPair {
            entity_a: *e1,
            entity_b: *e2,
        });
    }));

    return collision_pairs;
}

//Naively add every pairs to check
#[must_use]
pub(crate) fn compute_collision_pairs_rough_st(
    query: &BroadPhaseQueryAwake,
    entities: &Vec<Entity>,
) -> Vec<CollisionPair>
{
    let _span = info_span!("broad_phase", name = "compute rough collisions").entered();

    let mut collision_pairs = vec!();
    collision_pairs.extend(query.iter_combinations().collect::<Vec<_>>().iter().filter_map(|[(e1, _p1, t1, _a1), (e2, _p2, t2, _a2)]| {
        return Some(CollisionPair {
            entity_a: *e1,
            entity_b: *e2,
        });
    }));

    return collision_pairs;
}

//Use Sweep & Prune to create a vector of CollisionPair
#[must_use]
pub(crate) fn compute_collision_pairs_sap_mt(
    query: &BroadPhaseQueryAwake,
    entities: &Vec<Entity>,
) -> Vec<CollisionPair>
{
    let _span = info_span!("broad_phase", name = "compute SAP collisions").entered();

    let count = entities.len();

    //fixme
    let mut collision_pairs = vec!();
    collision_pairs.par_extend(entities.par_iter().enumerate().flat_map(|(i, &entity1)|
        {
            let (e1, _p1, t1, a1) = query.get(entity1).unwrap();

            let mut tmp_collision = Vec::<CollisionPair>::with_capacity(count - i - 1);
            for &entity2 in entities.iter().skip(i + 1)
            {
                let (e2, _p2, t2, a2) = query.get(entity2).unwrap();
                if t2.translation.x + a2.min.x > t1.translation.x + a1.max.x { break; };

                if check_collision(&a1, &t1, &a2, &t2)
                {
                    tmp_collision.push(CollisionPair {
                        entity_a: e1,
                        entity_b: e2,
                    });
                }
            }
            return tmp_collision;
        })
        .collect::<Vec<_>>());

    return collision_pairs;
}

//Use Sweep & Prune to create a vector of CollisionPair
#[must_use]
pub(crate) fn compute_collision_pairs_sap_st(
    query: &BroadPhaseQueryAwake,
    entities: &Vec<Entity>,
) -> Vec<CollisionPair>
{
    let _span = info_span!("broad_phase", name = "compute SAP collisions").entered();

    let count = entities.len();

    //fixme
    let mut collision_pairs = vec!();
    collision_pairs.extend(entities.iter().enumerate().flat_map(|(i, &entity1)|
        {
            let (e1, _p1, t1, a1) = query.get(entity1).unwrap();

            let mut tmp_collision = Vec::<CollisionPair>::with_capacity(count - i - 1);
            for &entity2 in entities.iter().skip(i + 1)
            {
                let (e2, _p2, t2, a2) = query.get(entity2).unwrap();
                if t2.translation.x + a2.min.x > t1.translation.x + a1.max.x { break; };

                if check_collision(&a1, &t1, &a2, &t2)
                {
                    tmp_collision.push(CollisionPair {
                        entity_a: e1,
                        entity_b: e2,
                    });
                }
            }
            return tmp_collision;
        })
        .collect::<Vec<_>>());

    return collision_pairs;
}
