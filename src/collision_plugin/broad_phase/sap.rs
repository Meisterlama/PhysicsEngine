use bevy::prelude::*;
use rayon::prelude::*;

use crate::aabb::check_collision;

use super::BroadPhaseData;
use super::BroadPhaseQueryAwake;
use super::CollisionPair;

//Use Sweep & Prune to create a vector of CollisionPair
pub fn compute_collision_pairs(query: &BroadPhaseQueryAwake, broad_phase_data: &mut BroadPhaseData) -> Vec<CollisionPair>
{
    let _span = info_span!("broad_phase", name = "compute SAP collisions").entered();

    let collision_pairs: Vec<CollisionPair> = broad_phase_data.sorted_entities.par_iter().enumerate().map(|(i, &entity1)|
        {
            let (e1, _p1, t1, a1) = query.get(entity1).unwrap();

            let mut tmp_collision = Vec::<CollisionPair>::new();
            for &entity2 in broad_phase_data.sorted_entities.iter().skip(i+1)
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
        }).flatten().collect();
    return collision_pairs;
}