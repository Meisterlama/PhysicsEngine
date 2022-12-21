use bevy::prelude::*;
use rayon::prelude::*;

use super::BroadPhaseQueryAwake;
use super::CollisionPair;

//Naively add every pairs to check
pub(crate) fn compute_collision_pairs(query: &BroadPhaseQueryAwake, out_collision_pair: &mut Vec<CollisionPair>)
{
    let span = info_span!("broad_phase", name = "compute rough collisions").entered();

    out_collision_pair.par_extend(query.iter_combinations().collect::<Vec<_>>().par_iter().filter_map(|[(e1, _p1, t1, _a1), (e2, _p2, t2, _a2)]| {
        return Some(CollisionPair {
            entity_a: *e1,
            entity_b: *e2,
        });
    }));
}