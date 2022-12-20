use bevy::prelude::*;
use rayon::prelude::*;

use crate::aabb::check_collision;

use super::BroadPhaseData;
use super::BroadPhaseQueryAwake;
use super::CollisionPair;

//Use Sweep & Prune to create a vector of CollisionPair
pub fn compute_collision_pairs(query: &BroadPhaseQueryAwake, broad_phase_data: &mut BroadPhaseData)
{
    let _span = info_span!("broad_phase", name = "compute SAP collisions").entered();

    broad_phase_data.collision_pairs.par_extend(broad_phase_data.sorted_entities.par_iter().enumerate().flat_map(|(i, &entity1)|
        {
            let (e1, _p1, t1, a1) = query.get(entity1).unwrap();

            let mut tmp_collision = Vec::<CollisionPair>::with_capacity(broad_phase_data.sorted_entities.len() - i - 1);
            for &entity2 in broad_phase_data.sorted_entities.iter().skip(i + 1)
            {
                let (e2, _p2, t2, a2) = query.get(entity2).unwrap();
                if t2.translate(&a2.min).x > t1.translate(&a1.max).x { break; };

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
}