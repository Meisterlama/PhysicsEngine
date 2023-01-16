use std::time::Instant;

use bevy::prelude::*;
use rayon::prelude::*;

use crate::collision_plugin::narrow_phase::sat;
use crate::collision_plugin::config::{CollisionConfig, NarrowPhaseType};
use crate::collision_plugin::data_structs::{BroadPhaseData, CollisionInfo, NarrowPhaseData};
use crate::collision_plugin::narrow_phase::gjk;
use crate::collision_plugin::polygon_component::PolygonComponent;
use crate::transform2d::Transform2d;

pub type NarrowPhaseQuery<'w, 's> = Query<
    'w, 's,
    (
        &'static mut PolygonComponent,
        &'static Transform2d
    )
>;

pub(crate) fn narrow_phase(
    query: NarrowPhaseQuery,
    mut narrow_phase_data: ResMut<NarrowPhaseData>,
    broad_phase_data: Res<BroadPhaseData>,
    config: Res<CollisionConfig>,
)
{
    let start = Instant::now();

    narrow_phase_data.collision_infos.clear();
    let _span = info_span!("narrow_phase", name = "dispatching").entered();

    let collision_infos = if config.phase_config.multithread_narrow_phase {
        match config.phase_config.narrow_phase_type {
            NarrowPhaseType::Disabled => vec!(),
            NarrowPhaseType::SAT => narrow_phase_sat_mt(&broad_phase_data, &query, config.phase_config.compute_info_collision),
            NarrowPhaseType::GJK => narrow_phase_gjk_mt(&broad_phase_data, &query, config.phase_config.compute_info_collision),
        }
    } else {
        match config.phase_config.narrow_phase_type {
            NarrowPhaseType::Disabled => vec!(),
            NarrowPhaseType::SAT => narrow_phase_sat_st(&broad_phase_data, &query, config.phase_config.compute_info_collision),
            NarrowPhaseType::GJK => narrow_phase_gjk_st(&broad_phase_data, &query, config.phase_config.compute_info_collision),
        }
    };



    narrow_phase_data.collided_entities.clear();
    for info in &collision_infos {
        let pair = info.collision_pair.unwrap();

        narrow_phase_data.collided_entities.insert(pair.entity_a);
        narrow_phase_data.collided_entities.insert(pair.entity_b);
    }
    narrow_phase_data.collision_infos = collision_infos;

    narrow_phase_data.time += Instant::now() - start;
}

pub(crate) fn narrow_phase_sat_st(broad_phase_data: &BroadPhaseData,
                                  query: &NarrowPhaseQuery,
                                  _compute_collision_infos: bool,
) -> Vec<CollisionInfo>
{
    let _span = info_span!("narrow_phase", name = "SAT").entered();

    let collision_infos = broad_phase_data.collision_pairs.iter().filter_map(
        |pair| {
            let (p1, t1) = query.get(pair.entity_a).unwrap();
            let (p2, t2) = query.get(pair.entity_b).unwrap();
            let collided = sat::check_collision(&p1, &t1, &p2, &t2);

            if collided == true {
                return Some(CollisionInfo::new(&pair));
            }
            return None;
        }
    ).collect::<Vec<_>>();

    return collision_infos;
}

pub(crate) fn narrow_phase_sat_mt(broad_phase_data: &BroadPhaseData,
                                  query: &NarrowPhaseQuery,
                                  _compute_collision_infos: bool,
) -> Vec<CollisionInfo>
{
    let _span = info_span!("narrow_phase", name = "SAT").entered();

    let collision_infos = broad_phase_data.collision_pairs.par_iter().filter_map(
        |pair| {
            let (p1, t1) = query.get(pair.entity_a).unwrap();
            let (p2, t2) = query.get(pair.entity_b).unwrap();
            let collided = sat::check_collision(&p1, &t1, &p2, &t2);

            if collided == true {
                return Some(CollisionInfo::new(&pair));
            }
            return None;
        }
    ).collect::<Vec<_>>();

    return collision_infos;
}

pub(crate) fn narrow_phase_gjk_mt(broad_phase_data: &BroadPhaseData,
                                  query: &NarrowPhaseQuery,
                                  compute_collision_infos: bool,
) -> Vec<CollisionInfo> {
    let _span = info_span!("narrow_phase", name = "GJK").entered();

    //fixme
    let collision_infos = broad_phase_data.collision_pairs.par_iter().filter_map(
        |pair| {
            let (p1, t1) = query.get(pair.entity_a).unwrap();
            let (p2, t2) = query.get(pair.entity_b).unwrap();
            let (collided, simplex) = gjk::check_collision(&p1, &t1, &p2, &t2);

            if collided == true {
                let mut collision_info = if compute_collision_infos { gjk::get_info_collisions(&p1, &t1, &p2, &t2, simplex) } else { CollisionInfo::default() };
                collision_info.collision_pair = Some(pair.clone());
                return Some(collision_info);
            }
            return None;
        }
    ).collect::<Vec<_>>();

    return collision_infos;
}

pub(crate) fn narrow_phase_gjk_st(broad_phase_data: &BroadPhaseData,
                                  query: &NarrowPhaseQuery,
                                  compute_collision_infos: bool,
) -> Vec<CollisionInfo> {
    let _span = info_span!("narrow_phase", name = "GJK").entered();

    //fixme
    let collision_infos = broad_phase_data.collision_pairs.iter().filter_map(
        |pair| {
            let (p1, t1) = query.get(pair.entity_a).unwrap();
            let (p2, t2) = query.get(pair.entity_b).unwrap();
            let (collided, simplex) = gjk::check_collision(&p1, &t1, &p2, &t2);

            if collided == true {
                let mut collision_info = if compute_collision_infos { gjk::get_info_collisions(&p1, &t1, &p2, &t2, simplex) } else { CollisionInfo::default() };
                collision_info.collision_pair = Some(pair.clone());
                return Some(collision_info);
            }
            return None;
        }
    ).collect::<Vec<_>>();

    return collision_infos;
}