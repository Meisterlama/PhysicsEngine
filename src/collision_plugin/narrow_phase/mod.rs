use std::time::{Duration, Instant};

use bevy::prelude::*;
use bevy::utils::HashSet;
use bevy_prototype_debug_lines::DebugLines;
use rayon::prelude::*;

use crate::aabb::AABB;
use crate::collision_plugin::broad_phase::BroadPhaseData;
use crate::collision_plugin::collision_structs::CollisionInfo;
use crate::collision_plugin::CollisionStage;
use crate::collision_plugin::config::{CollisionConfig, NarrowPhaseType};
use crate::polygon_component::PolygonComponent;
use crate::transform2d::Transform2d;

mod sat;
mod gjk;
pub mod helpers;

type NarrowPhaseQuery<'w, 's> = Query<'w, 's, (&'static mut PolygonComponent, &'static Transform2d)>;

pub struct NarrowPhasePlugin;

impl Plugin for NarrowPhasePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<NarrowPhaseData>()
            .add_system_to_stage(CollisionStage::NarrowPhase, narrow_phase);
    }
}

pub fn narrow_phase(
    query: NarrowPhaseQuery,
    mut narrow_phase_data: ResMut<NarrowPhaseData>,
    mut broad_phase_data: ResMut<BroadPhaseData>,
    config: Res<CollisionConfig>,
)
{

    let start = Instant::now();

    narrow_phase_data.collision_infos.clear();
    let span = info_span!("narrow_phase", name = "dispatching").entered();

    let collision_infos = match config.narrow_phase_type {
        NarrowPhaseType::Disabled => vec!(),
        NarrowPhaseType::SAT => narrow_phase_sat(&broad_phase_data, &query, config.compute_info_collision),
        NarrowPhaseType::GJK => narrow_phase_gjk(&broad_phase_data, &query, config.compute_info_collision),
    };


    narrow_phase_data.collided_entities.clear();
    for info in &collision_infos {
        let pair = info.collision_pair.unwrap();

        narrow_phase_data.collided_entities.insert(pair.entity_a);
        narrow_phase_data.collided_entities.insert(pair.entity_b);
    }
    narrow_phase_data.collision_infos = collision_infos;

    narrow_phase_data.time = Instant::now() - start;
}

fn narrow_phase_sat(broad_phase_data: &BroadPhaseData,
                    query: &NarrowPhaseQuery,
                    compute_collision_infos: bool,
) -> Vec<CollisionInfo>
{
    let span = info_span!("narrow_phase", name = "SAT").entered();

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

fn narrow_phase_gjk(broad_phase_data: &BroadPhaseData,
                    query: &NarrowPhaseQuery,
                    compute_collision_infos: bool,
) -> Vec<CollisionInfo> {
    let span = info_span!("narrow_phase", name = "GJK").entered();

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

#[derive(Default, Resource)]
pub struct NarrowPhaseData {
    pub collision_infos: Vec<CollisionInfo>,
    pub collided_entities: HashSet<Entity>,
    pub time: Duration,
}