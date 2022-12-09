use std::time::{Duration, Instant};

use bevy::prelude::*;
use bevy::utils::HashSet;
use rayon::prelude::*;

use crate::aabb::AABB;
use crate::collision_plugin::{CollisionStage};
use crate::collision_plugin::broad_phase::BroadPhaseData;
use crate::collision_plugin::collision_structs::{CollisionInfo, CollisionPair};
use crate::collision_plugin::config::{CollisionConfig, NarrowPhaseType};
use crate::collision_plugin::sat::check_collision;
use crate::polygon_component::PolygonComponent;
use crate::transform2d::Transform2d;

type NarrowPhaseQuery<'w, 's> = Query<'w, 's, (Entity, &'static mut PolygonComponent, &'static Transform2d, &'static AABB)>;

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
    config: Res<CollisionConfig>)
{
    let start = Instant::now();

    narrow_phase_data.collision_infos.clear();

    narrow_phase_data.collision_infos = match config.narrow_phase_type {
        NarrowPhaseType::Disabled => vec!(),
        NarrowPhaseType::Enabled => narrow_phase_precise(&broad_phase_data, &query)
    };

    narrow_phase_data.collided_entities.clear();
    let mut tmp_hashset = HashSet::new();
    for info in &narrow_phase_data.collision_infos {
        let pair = info.collision_pair.unwrap();

        tmp_hashset.insert(pair.entity_a);
        tmp_hashset.insert(pair.entity_b);
    }

    narrow_phase_data.collided_entities = tmp_hashset;

    narrow_phase_data.time = Instant::now() - start;
}

fn narrow_phase_precise(broad_phase_data: &BroadPhaseData,
                        query: &NarrowPhaseQuery,
) -> Vec<CollisionInfo>
{

    let collision_infos = broad_phase_data.collision_pairs.par_iter().filter_map(
        |pair| {
            let (e1, p1, t1, a1) = query.get(pair.entity_a).unwrap();
            let (e2, p2, t2, a2) = query.get(pair.entity_b).unwrap();
            let (collided, collision_info) = check_collision(&p1, &t1, &p2, &t2);

            if collided == true {
                return Some(CollisionInfo{
                    collision_pair: Some(pair.clone()),
                    location: Default::default(),
                    normal: Default::default(),
                    distance: 0.0,
                })

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