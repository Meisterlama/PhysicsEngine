use std::time::{Duration, Instant};

use bevy::prelude::*;

use crate::aabb::AABB;
use crate::collision_plugin::{CollisionStage, PhysicsAwake};
use crate::collision_plugin::collision_structs::CollisionPair;
use crate::collision_plugin::config::{BroadPhaseType, CollisionConfig};
use crate::polygon_component::PolygonComponent;
use crate::transform2d::Transform2d;

mod rough;
mod sap;

pub type BroadPhaseQueryAwake<'w, 's> = Query<'w, 's, (Entity, &'static PolygonComponent, &'static Transform2d, &'static AABB), With<PhysicsAwake>>;

pub struct BroadPhasePlugin;

impl Plugin for BroadPhasePlugin
{
    fn build(&self, app: &mut App) {
        app
            .init_resource::<BroadPhaseData>()
            .add_system_to_stage(CollisionStage::BroadPhase,broad_phase);
    }
}

pub fn broad_phase(query: BroadPhaseQueryAwake,
                   mut broad_phase_data: ResMut<BroadPhaseData>,
                   config: ResMut<CollisionConfig>)
{
    let span = info_span!("broad_phase", name = "dispatching").entered();

    let start = Instant::now();
    broad_phase_data.collision_pairs.clear();

    match config.broad_phase_type {
        BroadPhaseType::Disabled => {}
        BroadPhaseType::Rough => {
            rough::compute_collision_pairs(&query, &mut broad_phase_data.collision_pairs);
        }
        BroadPhaseType::SAP => {
            sap::compute_collision_pairs(&query, &mut broad_phase_data);
        }
    }

    broad_phase_data.time = Instant::now() - start;
}

#[derive(Default, Resource)]
pub struct BroadPhaseData {
    pub collision_pairs: Vec<CollisionPair>,
    pub sorted_entities: Vec<Entity>,
    pub time: Duration,
}