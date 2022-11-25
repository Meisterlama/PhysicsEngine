use std::time::Instant;
use bevy::prelude::*;

use rayon::prelude::*;
use crate::aabb::AABB;

use crate::collision_plugin::SystemData;
use crate::collision_plugin::collision_structs::{CollisionInfo, CollisionPair};
use crate::collision_plugin::config::{CollisionConfig, NarrowPhaseType};
use crate::collision_plugin::sat::check_collision;
use crate::polygon_component::PolygonComponent;
use crate::transform2d::Transform2d;

type NarrowPhaseQuery<'w, 's> = Query<'w, 's, (Entity, &'static mut PolygonComponent, &'static Transform2d, &'static AABB)>;


pub fn narrow_phase(
    query: NarrowPhaseQuery,
    mut system_data: ResMut<SystemData>,
    config: Res<CollisionConfig>)
{
    let start = Instant::now();

    if let Some(collision_infos) = &mut system_data.narrow_phase_collision_infos
    {
        collision_infos.clear();
    }

    system_data.narrow_phase_collision_infos = match config.narrow_phase_type {
        NarrowPhaseType::Disabled => None,
        NarrowPhaseType::Enabled => Some(narrow_phase_precise(&system_data, &query))
    };

    system_data.narrow_time = Instant::now() - start;
}

fn narrow_phase_precise(entities_to_check: &SystemData,
                            query: &NarrowPhaseQuery,
) -> Vec<CollisionInfo>
{
    if let Some(collision_pairs) = &entities_to_check.broad_phase_collision_pairs {
        let entities_to_query: Vec<Entity> = collision_pairs.par_iter().flat_map(|pair| [pair.entity_a.unwrap(), pair.entity_b.unwrap()]).collect();

        let collision_infos = query.iter_many(&entities_to_query).array_chunks::<2>().collect::<Vec<_>>().par_iter().filter_map(
            |[(e1, p1, t1, _a1), (e2, p2, t2, _a2)]| {
                let (collided, collision_info) = check_collision(&p1, &t1, &p2, &t2);
                if collided == true
                {
                    return Some(CollisionInfo {
                        pair: CollisionPair{
                            entity_a: Some(*e1),
                            pos_a: t1.position,
                            entity_b: Some(*e2),
                            pos_b: t2.position
                        },
                        location: Default::default(),
                        normal: Default::default(),
                        distance: 0.0,
                    });
                }
                return None;
            }
        ).collect();
        return collision_infos;
    }
    vec!()
}